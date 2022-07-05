import './App.css';
import { useEffect, useRef, useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { getParsedNftAccountsByOwner, isValidSolanaAddress, createConnectionConfig, } from "@nfteyez/sol-rayz";
import { Col, Row, Button, Form, Card, Badge } from "react-bootstrap";
import AlertDismissible from './alert/alertDismissible';
import axios from "axios";
import { Connection } from '@solana/web3.js';
import {
  Provider
} from '@project-serum/anchor';
import styles from "./address.module.css";
import { BounceLoader } from "react-spinners";
import {
  initialize, _getState, addOgList, addWlList, addBlList, removeBlList, _updateOgList,
  removeOgList, removeWlList, updatePrice, setStage, updateAmount, multiMint, setUri, isOgList
} from './utils';
const opts = {
  preflightCommitment: "processed"
}
function App(props) {
  const { publicKey } = useWallet();
  const { connection } = props;
  const wallet = useWallet();
  // input ref
  const inputRef = useRef();
  const ogkey = useRef();
  const wlkey = useRef();
  const blkey = useRef();
  const og_price = useRef();
  const wl_price = useRef();
  const bl_price = useRef();
  const og_amount = useRef();
  const wl_amount = useRef();
  const bl_amount = useRef();
  const cur_stage = useRef();
  const mint_cnt = useRef();
  const base_uri = useRef();


  // state change
  useEffect(() => {
    setNfts([]);
    setView("collection");
    setGroupedNfts([]);
    setShow(false);
    if (publicKey) {
      inputRef.current.value = publicKey;
      getState();
    }
  }, [publicKey, connection]);

  const [nfts, setNfts] = useState([]);
  const [groupedNfts, setGroupedNfts] = useState([]);
  const [view, setView] = useState('collection');
  //alert props
  const [title, setTitle] = useState("");
  const [message, setMessage] = useState("");
  const [show, setShow] = useState(false);

  //loading props
  const [loading, setLoading] = useState(false);

  const [curStage, setCurStage] = useState('');
  const [mintable, setMintable] = useState(false);
  const [price, setPrice] = useState(0);
  const [ogList, setOgList] = useState([]);
  const [wlList, setWlList] = useState([]);

  const [whitelist, setWhitelist] = useState(" ");
  const [ogListUrl, setOgListUrl] = useState("");
  const [ogRootUrl, setOgRootUrl] = useState("");
  const [proofArry, setProofArry] = useState([]);

  const generateMerkleProof = async (whitelist) => {
    setLoading(true);
    try {
      console.log(whitelist, "WHITE LIST str -------");
      let doubleQuotesRemoved = whitelist.replaceAll('"', "");
      let singleQuotesRemoved = doubleQuotesRemoved.replaceAll("'", "");
      let lineBreaksRemoved = singleQuotesRemoved.replaceAll(/\r?\n|\r/g, "");
      let spacesRemoved = lineBreaksRemoved.replaceAll(" ", "").trim();
      const whitelistArray = spacesRemoved.split(",");
      console.log(whitelistArray, "WHITE LIST ARRAY ][][][][][");
      let data = {
        collectionName: "collectionName",
        userEmail: "example@gmail.com",
        data: whitelistArray,
      };
      const response = await axios.post(
        "https://merklemeapi.vincanger.repl.co/merkleTree/generate ",
        data
      );
      const dataToPassToNextPage = response.data;
      await updateOgInfo(dataToPassToNextPage);  
      console.log(dataToPassToNextPage);
      setTimeout(() => {
        setLoading(false);
      }, 1000);
    } catch (error) {
      console.error(error);
      setLoading(false);
    }
  };

  const updateOgInfo = async (data) => {
    try{
      const provider = await getProvider();
      const og_list_url = data.whitelist;
      const og_root_url = data.rootHash;
      const og_hash_res = await fetch(og_root_url);
      const og_hash_info = await og_hash_res.json();
      const og_hash_str = og_hash_info.rootHash;
      let og_hex_str = og_hash_str.replaceAll("0x", "");
      const og_hash = Buffer.from(og_hex_str, "hex");
      console.log(og_list_url, og_hash, og_hash_info.rootHash, og_root_url);
      await _updateOgList(provider, wallet, og_list_url, og_root_url, og_hash);
      alert("Success!!!");
    }catch{
      alert("Fail...");
    }
  }

  const getNfts = async (e) => {
    e.preventDefault();

    setShow(false);

    let address = inputRef.current.value;

    if (address.length === 0) {
      address = publicKey;
    }

    if (!isValidSolanaAddress(address)) {
      setTitle("Invalid address");
      setMessage("Please enter a valid Solana address or Connect your wallet");
      setLoading(false);
      setShow(true);
      return;
    }

    const connect = createConnectionConfig(connection);

    setLoading(true);
    const nftArray = await getParsedNftAccountsByOwner({
      publicAddress: address,
      connection: connect,
      serialization: true,
    });


    if (nftArray.length === 0) {
      setTitle("No NFTs found in " + props.title);
      setMessage("No NFTs found for address: " + address);
      setLoading(false);
      setView('collection');
      setShow(true);
      return;
    }

    const metadatas = await fetchMetadata(nftArray);
    var group = {};

    for (const nft of metadatas) {
      if (group.hasOwnProperty(nft.data.symbol)) {
        group[nft.data.symbol].push(nft);
      } else {
        group[nft.data.symbol] = [nft];
      }
    }
    setGroupedNfts(group);

    setLoading(false);
    return setNfts(metadatas);
  };

  const fetchMetadata = async (nftArray) => {
    let metadatas = [];
    for (const nft of nftArray) {
      try {
        await fetch(nft.data.uri)
          .then((response) => response.json())
          .then((meta) => {
            metadatas.push({ ...meta, ...nft });
          });
      } catch (error) {
        console.log(error);
      }
    }
    return metadatas;
  };

  const getProvider = async () => {
    /* create the provider and return it to the caller */
    /* network set to local network for now */
    const network = "https://metaplex.devnet.rpcpool.com";
    const connection = new Connection(network, opts.preflightCommitment);

    const provider = new Provider(
      connection, wallet, opts.preflightCommitment,
    );
    return provider;
  }

  const getProof = async (ogListUrl, ogRootUrl) => {
    const leafToVerify = wallet.publicKey.toBase58();
    const verifyData = {
      whitelist: ogListUrl,
      rootHash: ogRootUrl,
      leafToVerify: leafToVerify,
    };
    try {
      const response = await axios.post(
        "https://merklemeApi.vincanger.repl.co/verify/proof",
        verifyData
      );
      const proofs = response.data.proof;
      const proofAry = [];
      for(const proof of proofs) {
        let proofStr = proof.replaceAll("0x", "");
        let tmp = Buffer.from(proofStr, "hex");
        proofAry.push(tmp);
      }
      setProofArry(proofAry);
      console.log("GetProof", "Success");
    }catch (error) {
      console.error("GetProof",error);
    }
  }

  async function test() {
    const provider = await getProvider();
    const leafToVerify = wallet.publicKey.toBase58();
    const verifyData = {
      whitelist: ogListUrl,
      rootHash: ogRootUrl,
      leafToVerify: leafToVerify,
    };
    console.log(verifyData, wallet.publicKey.toBase58());
    try {
      const response = await axios.post(
        "https://merklemeApi.vincanger.repl.co/verify/proof",
        verifyData
      );
      const proofs = response.data.proof;
      const proofAry = [];
      for(const proof of proofs) {
        let proofStr = proof.replaceAll("0x", "");
        let tmp = Buffer.from(proofStr, "hex");
        proofAry.push(tmp);
      }
      await isOgList(provider, wallet, proofAry);
      console.log("proof", "Success");
    }catch (error) {
      console.error(error);
    }
  }

  async function nftMint() {
    const provider = await getProvider();
    if(mint_cnt.current.value <= 0) {
      alert("Mint Count is none.");
      return;
    }
    
    try{
      const r = await multiMint(provider, wallet,price, mint_cnt.current.value, proofArry);
      console.log('rrr', r);
      if(r){
        alert("Success!!!");
      }
      else { 
        alert("Fail");  
      }
    } catch (e) {
      alert("Fail:", e);
    }
  }

  async function init() {
    const provider = await getProvider();
    await initialize(provider, wallet);
  }

  async function addOg() {
    const provider = await getProvider();
    try{
      if(ogList.indexOf(ogkey.current.value) >= 0) {
        alert("This user already is in OG List.");
        return;
      }
      if(wlList.indexOf(ogkey.current.value) >= 0) {
        alert("This user already is in WL List.");
        return;
      }
      await addOgList(provider, wallet, [ogkey.current.value]);
      alert("Success!!!");
      getState();
    } catch (e) {
      alert("Fail:", e);
    }
  }

  async function removeOg() {
    const provider = await getProvider();
    try{
      if(ogList.indexOf(ogkey.current.value) < 0) {
        alert("This user is not in OG List.");
        return;
      }
      await removeOgList(provider, wallet, [ogkey.current.value]);
      alert("Success!!!");
      getState();
    } catch (e) {
      alert("Fail:", e);
    }
  }

  async function addWl() {
    const provider = await getProvider();
    const wladdresses = wlkey.current.value.split(',');
    try {    
      // await addWlList(provider, wallet, [wlkey.current.value]);
      if(ogList.indexOf(wlkey.current.value) >= 0) {
        alert("This user already is in OG List.");
        return;
      }
      if(wlList.indexOf(wlkey.current.value) >= 0) {
        alert("This user already is in WL List.");
        return;
      }
      await addWlList(provider, wallet, wladdresses);
      alert("Success!!!");
      getState();
    } catch (e) {
      alert("Fail:", e);
    }
  }

  async function removeWl() {
    const provider = await getProvider();
    try{
      if(wlList.indexOf(wlkey.current.value) < 0) {
        alert("This user is not in WL List.");
        return;
      }
      await removeWlList(provider, wallet, [wlkey.current.value]);
      alert("Success!!!");
      getState();
    } catch (e) {
      alert("Fail:", e);
    }
  }

  async function addBl() {
    const provider = await getProvider();
    await addBlList(provider, wallet, [blkey.current.value]);
  }

  async function removeBl() {
    const provider = await getProvider();
    await removeBlList(provider, wallet, [blkey.current.value]);
  }

  async function changePrice() {
    const provider = await getProvider();
    try{
      await updatePrice(provider, wallet, og_price.current.value * 1000, wl_price.current.value * 1000, bl_price.current.value * 1000);
      alert("Success!!!");
      getState();
    } catch (e) {
      alert("Fail:", e);
    }
  }

  async function changeAmount() {
    const provider = await getProvider();
    try{
      await updateAmount(provider, wallet, og_amount.current.value * 1, wl_amount.current.value * 1, bl_amount.current.value * 1);
      alert("Success!!!");
      getState();
    } catch (e) {
      alert("Fail:", e);
    }
  }

  async function changeStage() {
    const provider = await getProvider();
    try{
      await setStage(provider, wallet, cur_stage.current.value * 1);
      alert("Success!!!");
      getState();
    } catch (e) {
      alert("Fail:", e);
    }
  }

  async function changeUri() {
    const provider = await getProvider();
    try{
      await setUri(provider, wallet, base_uri.current.value);
      alert("Success!!!");
      getState();
    } catch (e) {
      alert("Fail:", e);
    }
  }

  async function getState() {
    const provider = await getProvider();
    const cur = await _getState(provider, wallet);
    let sta = "PublicList";
    if (cur.stage == 1) sta = "OGList";
    if (cur.stage == 2) sta = "WhiteList";
    if (cur.stage == 4) sta = "BlackList";
    setCurStage(sta);
    setPrice(cur.price / 1000);
    setMintable(true);
    cur_stage.current.value = cur.stage;
    og_price.current.value = cur.ogPrice / 1000;
    wl_price.current.value = cur.wlPrice / 1000;
    bl_price.current.value = cur.blPrice / 1000;
    og_amount.current.value = cur.ogAmout;
    wl_amount.current.value = cur.wlAmout;
    bl_amount.current.value = cur.blAmout;
    base_uri.current.value = cur.baseUri;
    setOgList(cur.ogList);
    setWlList(cur.wlList);
    setOgListUrl(cur.ogListUrl);
    setOgRootUrl(cur.ogRootUrl);
    await getProof(cur.ogListUrl, cur.ogRootUrl);
  }

  return (
    <div className="main">
      <Row>
        <Col lg='3'></Col>
        <Col>
          <h4>CurrentStage: {curStage}</h4>
        </Col>
        <Col>
          <h4>Price: {price}</h4>
        </Col>
        <Col lg='2'>
          <Button onClick={getNfts}>Show NFTs</Button>
        </Col>
        <Col lg='1'>
          <Button onClick={test}>test</Button>
        </Col>
      </Row>
      <Row className="inputForm">
        <Col xs="12" md="12" lg="5">
          <Form.Control
            type="text"
            readOnly
            ref={inputRef}
            placeholder="Wallet address"
          />
        </Col>
        <Col xs="12" md="12" lg="5" className="d-grid">
          {mintable && (
            <Row>
              <Col lg="1">
                <Button onClick={()=>{
                  mint_cnt.current.value = mint_cnt.current.value*1+1;
                }}>+</Button>
              </Col>
              <Col lg="2">
                <Form.Control
                  type="text"
                  ref={mint_cnt}
                  // value={mint_cnt.current.value}
                />
              </Col>
              <Col lg="1">
                <Button onClick={()=>{
                  mint_cnt.current.value = mint_cnt.current.value*1-1;
                }}>-</Button>
              </Col>
              <Col lg="2">
                <Button
                  variant={props.variant.toLowerCase()}
                  type="submit"
                  onClick={nftMint}
                >
                  Mint
                </Button>
              </Col>
            </Row>
          )}
        </Col>
        <Col lg="1">
          {view === "nft-grid" && (
            <Button
              size="md"
              variant="danger"
              onClick={() => {
                setView("collection");
              }}
            >
              Close
            </Button>
          )}
        </Col>
      </Row>
      <Row className="inputForm">
        <Col lg="4">
          <Form.Control
            type="text"
            ref={ogkey}
            placeholder="OGList Wallet Address"
          />
        </Col>
        <Col sm="1" className="d-grid">
          <Button
            type="submit"
            onClick={addOg}
          >Add
          </Button>
        </Col>
        <Col sm="1" className="d-grid">
          <Button
            type="submit"
            onClick={removeOg}
          >Remove
          </Button>
        </Col>
      </Row>
      <Row className='inputForm'>
        <Col lg="4">
          <Form.Control
            type="text"
            ref={wlkey}
            placeholder="WLList Wallet Address"
          />
        </Col>
        <Col sm="1" className="d-grid">
          <Button
            type="submit"
            onClick={addWl}
          >Add
          </Button>
        </Col>
        <Col sm="1" className="d-grid">
          <Button
            type="submit"
            onClick={removeWl}
          >Remove
          </Button>
        </Col>
      </Row>
      {/* <Row className='inputForm'>  
        <Col lg="4">
          <Form.Control
            type="text"
            ref={blkey}
            placeholder="BLList Wallet Address"
          />
        </Col>
        <Col sm="1" className="d-grid">
          <Button
            type="submit"
            onClick={addBl}
          >Add
          </Button>
        </Col>
        <Col sm="1" className="d-grid">
          <Button
            type="submit"
            onClick={removeBl}
          >Remove
          </Button>
        </Col>
      </Row> */}
      <Row className="inputForm">
        <Col lg="1">
          <Form.Control
            type="text"
            ref={og_price}
            placeholder="OGPric"
          />
        </Col>
        <Col lg="1">
          <Form.Control
            type="text"
            ref={wl_price}
            placeholder="WLPric"
          />
        </Col>
        <Col lg="1">
          <Form.Control
            type="text"
            ref={bl_price}
            placeholder="PLPric"
          />
        </Col>
        <Col lg="1" className="d-grid">
          <Button
            type="submit"
            onClick={changePrice}
          >Update
          </Button>
        </Col>
      </Row>

      <Row className="inputForm">
        <Col lg="1">
          <Form.Control
            type="text"
            ref={og_amount}
            placeholder="OGMax"
          />
        </Col>
        <Col lg="1">
          <Form.Control
            type="text"
            ref={wl_amount}
            placeholder="WLMax"
          />
        </Col>
        <Col lg="1">
          <Form.Control
            type="text"
            ref={bl_amount}
            placeholder="PLMax"
          />
        </Col>
        <Col lg="1" className="d-grid">
          <Button
            type="submit"
            onClick={changeAmount}
          >Update
          </Button>
        </Col>
      </Row>

      <Row className="inputForm">
        <Col lg="3">
        <Form.Control
            type="text"
            ref={base_uri}
            placeholder="Base URI"
          />
        </Col>
        <Col lg="1" className="d-grid">
          <Button
            type="submit"
            onClick={changeUri}
          >Set
          </Button>
        </Col>
      </Row>

      <Row className="inputForm">
        <Col lg="1">
          <select className='form-control' ref={cur_stage}>
            <option value='1'>OGLIST</option>
            <option value='2'>WHITELIST</option>
            <option value='3'>PUBLIC</option>
          </select>
        </Col>
        <Col lg="1" className="d-grid">
          <Button
            type="submit"
            onClick={changeStage}
          >Set
          </Button>
        </Col>
      </Row>
      
      <Row>
        <div className={styles.addressFormSection}>
            <textarea
              placeholder=""
              rows="20"
              cols="40"
              className={styles.textarea}
              value={whitelist}
              onChange={(e) => setWhitelist(e.target.value)}
            ></textarea>

            <div className={styles.buttonWrapper}>
              <button
                onClick={() => generateMerkleProof(whitelist)}
                className={styles.nextStyles}
              >
                Merkle Me!
                <BounceLoader size={20} color={"#21327d"} loading={loading} />
              </button>
            </div>
          </div>
      </Row>
      {loading && (
        <div className="loading">
          <img src="loading.gif" alt="loading" />
        </div>
      )}

      <Row>
        {!loading &&
          view === "collection" &&
          Object.keys(groupedNfts).map(
            (metadata, index) => (
              (
                <Col xs="12" md="6" lg="2" key={index}>
                  <Card
                    onClick={() => {
                      setNfts(groupedNfts[metadata]);
                      setView("nft-grid");
                    }}
                    className="imageGrid"
                    lg="3"
                    style={{
                      width: "100%",
                      backgroundColor: "#2B3964",
                      padding: "10px",
                      borderRadius: "10px",
                    }}
                  >
                    <Card.Img
                      variant="top"
                      src={groupedNfts[metadata][0]?.image}
                      alt={groupedNfts[metadata][0]?.name}
                      style={{
                        borderRadius: "10px",
                      }}
                    />
                    <Card.Body>
                      <span>
                        <Card.Title style={{ color: "#fff" }}>
                          {metadata}
                        </Card.Title>
                        <Badge
                          pill
                          bg={props.variant.toLowerCase()}
                          text="light"
                        >
                          <h6>{groupedNfts[metadata].length}</h6>
                        </Badge>
                      </span>
                    </Card.Body>
                  </Card>
                </Col>
              )
            )
          )}
      </Row>

      {
        <Row>
          {!loading &&
            view === "nft-grid" &&
            nfts.map((metadata, index) => (
              <Col xs="12" md="6" lg="2" key={index}>
                <Card
                  onClick={() => {
                  }}
                  className="imageGrid"
                  lg="3"
                  style={{
                    width: "100%",
                    backgroundColor: "#2B3964",
                    padding: "10px",
                    borderRadius: "10px",
                  }}
                >
                  <Card.Img
                    variant="top"
                    src={metadata?.image}
                    alt={metadata?.name}
                  />
                  <Card.Body>
                    <Card.Title style={{ color: "#fff" }}>
                      {metadata?.name}
                    </Card.Title>
                  </Card.Body>
                </Card>
              </Col>
            ))}
        </Row>
      }

      {show && (
        <AlertDismissible title={title} message={message} setShow={setShow} />
      )}
    </div>
  );
}

export default App;
