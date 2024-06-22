import React, { useState, useEffect } from 'react';
import Head from 'next/head';
import Link from 'next/link';
import { Container, Typography, Grid, Card, CardContent, Button, makeStyles } from '@material-ui/core';
import { getPools } from '../utils/xionUtil';  // Utility function to fetch pools

const useStyles = makeStyles((theme) => ({
  root: {
    padding: theme.spacing(3),
  },
  card: {
    marginBottom: theme.spacing(2),
  },
  button: {
    marginTop: theme.spacing(1),
  },
}));

const PoolsPage = () => {
  const classes = useStyles();
  const [pools, setPools] = useState([]);

  useEffect(() => {
    const fetchPools = async () => {
      const data = await getPools();
      setPools(data);
    };
    fetchPools();
  }, []);

  return (
    <>
      <Head>
        <title>Pools - Dexter Swap</title>
        <meta name="description" content="View all available liquidity pools on Dexter Swap" />
      </Head>
      <Container className={classes.root}>
        <Typography variant="h4" gutterBottom>Liquidity Pools</Typography>
        <Grid container spacing={3}>
          {pools.map((pool) => (
            <Grid item xs={12} sm={6} md={4} key={pool.id}>
              <Card className={classes.card}>
                <CardContent>
                  <Typography variant="h6">{pool.name}</Typography>
                  <Typography variant="body2">Token Pair: {pool.tokenPair}</Typography>
                  <Typography variant="body2">Total Liquidity: {pool.totalLiquidity}</Typography>
                  <Button
                    variant="contained"
                    color="primary"
                    className={classes.button}
                    component={Link}
                    href={`/pools/${pool.id}`}
                  >
                    View Details
                  </Button>
                </CardContent>
              </Card>
            </Grid>
          ))}
        </Grid>
      </Container>
    </>
  );
};

export default PoolsPage;
