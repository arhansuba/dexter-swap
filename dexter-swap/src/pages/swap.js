import React from 'react';
import Head from 'next/head';
import { Container, Typography } from '@material-ui/core';
import SwapComponent from '../components/Swap/SwapComponent';

const tokens = ['TokenA', 'TokenB', 'TokenC']; // Example tokens, replace with actual token list

const SwapPage = () => {
  return (
    <>
      <Head>
        <title>Swap Tokens</title>
        <meta name="description" content="Swap tokens on Dexter Swap" />
      </Head>
      <Container maxWidth="md">
        <Typography variant="h4" align="center" gutterBottom>
          Swap Tokens
        </Typography>
        <SwapComponent tokens={tokens} />
      </Container>
    </>
  );
};

export default SwapPage;
