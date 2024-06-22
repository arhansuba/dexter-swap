import React from 'react';
import Head from 'next/head';
import Link from 'next/link';
import { Container, Typography, Button, Grid, makeStyles } from '@material-ui/core';
import { useAbstraxionAccount, useAbstraxionSigningClient } from '@burnt-labs/abstraxion';

const useStyles = makeStyles((theme) => ({
  root: {
    padding: theme.spacing(3),
    textAlign: 'center',
  },
  button: {
    margin: theme.spacing(2),
  },
}));

const HomePage = () => {
  const classes = useStyles();
  const { account } = useAbstraxionAccount();
  const { connect } = useAbstraxionSigningClient();

  const handleConnect = async () => {
    try {
      await connect();
    } catch (error) {
      console.error("Connection failed", error);
    }
  };

  return (
    <>
      <Head>
        <title>Dexter Swap</title>
        <meta name="description" content="Welcome to Dexter Swap, a decentralized exchange" />
      </Head>
      <Container maxWidth="md" className={classes.root}>
        <Typography variant="h3" gutterBottom>
          Welcome to Dexter Swap
        </Typography>
        <Typography variant="h5" gutterBottom>
          A decentralized exchange on Xion
        </Typography>
        <Grid container spacing={3} justifyContent="center">
          <Grid item>
            <Button
              variant="contained"
              color="primary"
              className={classes.button}
              component={Link}
              href="/swap"
            >
              Swap Tokens
            </Button>
          </Grid>
          <Grid item>
            <Button
              variant="contained"
              color="secondary"
              className={classes.button}
              component={Link}
              href="/liquidity"
            >
              Manage Liquidity
            </Button>
          </Grid>
        </Grid>
        {!account && (
          <Button
            variant="contained"
            color="default"
            className={classes.button}
            onClick={handleConnect}
          >
            Connect Wallet
          </Button>
        )}
        {account && (
          <Typography variant="body1">
            Connected as: {account.address}
          </Typography>
        )}
      </Container>
    </>
  );
};

export default HomePage;

