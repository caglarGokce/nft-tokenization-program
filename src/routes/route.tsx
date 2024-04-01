import { useWallet } from "@solana/wallet-adapter-react";
import { Connection, clusterApiUrl } from "@solana/web3.js";
import React from "react";
import { Route, Routes } from "react-router-dom";

import TokenizedSaleNFT from "../tokenizedonsale";
import WholeSaleNFT from "../wholeonsale";
import Fundraisings from "../fundraisings";
import MyTokenizedNFTs from "../mytokenizednfts";

const AppRoutes: React.FC = () => {
  const wallet = useWallet();

  const connection = new Connection(clusterApiUrl("testnet"));
  const { publicKey, sendTransaction } = useWallet();

  return (
    <Routes>
      <Route path="/mytokenizednfts" element={<MyTokenizedNFTs />} />
      <Route path="/fundraisings" element={<Fundraisings wallet={wallet} />} />
      <Route path="/mytokenizednfts" element={<MyTokenizedNFTs />} />
      <Route path="/wholeonsale" element={<WholeSaleNFT wallet={wallet} />} />
      <Route
        path="/tokenizedonsale"
        element={<TokenizedSaleNFT wallet={wallet} />}
      />
    </Routes>
  );
};

export default AppRoutes;
