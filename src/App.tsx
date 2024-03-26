
import { WalletAdapterNetwork, WalletNotConnectedError } from '@solana/wallet-adapter-base';
import { ConnectionProvider, WalletProvider, useConnection, useWallet } from '@solana/wallet-adapter-react';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { Button } from '@solana/wallet-adapter-react-ui/lib/types/Button';

import '../src/css/bootstrap.css'
import {
    GlowWalletAdapter,
    LedgerWalletAdapter,
    PhantomWalletAdapter,
    SlopeWalletAdapter,
    SolflareWalletAdapter,
    SolletExtensionWalletAdapter,
    SolletWalletAdapter,
    TorusWalletAdapter,

} from '@solana/wallet-adapter-wallets';


import { clusterApiUrl, Transaction, SystemProgram, Keypair, LAMPORTS_PER_SOL, PublicKey, Connection } from '@solana/web3.js';
import React, { FC, ReactNode, useMemo, useCallback, useState } from 'react';

import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import TokenizedSaleNFT from './tokenizedonsale';
import WholeSaleNFT from './wholeonsale';
import Fundraisings from './fundraisings';
import MyTokenizedNFTs from './mytokenizednfts';


require('./App.css');
require('@solana/wallet-adapter-react-ui/styles.css');


const App: FC = () => {

    return (
        <Router>
        <Context>
          <Content />
        </Context>
      </Router>
    );
};
export default App;

const Context: FC<{ children: ReactNode }> = ({ children }) => {

    const network = WalletAdapterNetwork.Testnet;
    const endpoint = useMemo(() => clusterApiUrl(network), [network]);
    const wallets = useMemo(
        () => [
            new LedgerWalletAdapter(),
            new PhantomWalletAdapter(),
            new GlowWalletAdapter(),
            new SlopeWalletAdapter(),
            new SolletExtensionWalletAdapter(), 
            new SolletWalletAdapter(),
            new SolflareWalletAdapter({ network }),
            new TorusWalletAdapter(),
        ],
        [network]
    );

    return (
        <ConnectionProvider endpoint={endpoint}>
            <WalletProvider wallets={wallets} autoConnect>
                <WalletModalProvider>{children}</WalletModalProvider>
            </WalletProvider>
        </ConnectionProvider>
    );
};

const Content: FC = () => {

    const wallet = useWallet();

    const connection = new Connection(clusterApiUrl("testnet"))
    const { publicKey, sendTransaction } = useWallet();


    return (
        <div className="App">

        <li><WalletMultiButton /></li>

        <nav>
          <Link to="/mytokenizednfts">MyTokenizedNFTs</Link>
          <Link to="/fundraisings">Fundraisings</Link>
          <Link to="/wholeonsale">WholeSaleNFT</Link>
          <Link to="/tokenizedonsale">TokenizedSaleNFT</Link>
        </nav>

        <Routes>
            <Route path="/mytokenizednfts" element={<MyTokenizedNFTs />} />
            <Route path="/fundraisings" element={<Fundraisings wallet={wallet}/>} />
           <Route path="/mytokenizednfts" element={<MyTokenizedNFTs  />} />
           <Route path="/wholeonsale" element={<WholeSaleNFT wallet={wallet}/>} />
            <Route path="/tokenizedonsale" element={<TokenizedSaleNFT wallet={wallet}/>} />
         </Routes>
        
        </div>
    );
};

