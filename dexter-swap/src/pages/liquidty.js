import React from 'react';
import Head from 'next/head';
import { Container, Typography } from '@material-ui/core';
import LiquidityComponent from '../components/Liquidity/LiquidityComponent';

const tokens = ['TokenA', 'TokenB', 'TokenC']; // Example tokens, replace with actual token list

const LiquidityPage = () => {
  return (
    <>
      <Head>
        <title>Manage Liquidity</title>
        <meta name="description" content="Add or remove liquidity on Dexter Swap" />
      </Head>
      <Container maxWidth="md">
        <Typography variant="h4" align="center" gutterBottom>
          Manage Liquidity
        </Typography>
        <LiquidityComponent tokens={tokens} />
      </Container>
    </>
  );
};

export default LiquidityPage;
