import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/router';
import Head from 'next/head';
import { Container, Typography, Grid, Card, CardContent, makeStyles } from '@material-ui/core';
import { getPoolDetails } from '../../utils/xionUtil';  // Utility function to fetch pool details

const useStyles = makeStyles((theme) => ({
  root: {
    padding: theme.spacing(3),
  },
  card: {
    marginBottom: theme.spacing(2),
  },
}));

const PoolDetailsPage = () => {
  const classes = useStyles();
  const router = useRouter();
  const { id } = router.query;

  const [pool, setPool] = useState(null);

  useEffect(() => {
    if (id) {
      const fetchPoolDetails = async () => {
        const data = await getPoolDetails(id);
        setPool(data);
      };
      fetchPoolDetails();
    }
  }, [id]);

  if (!pool) {
    return <Typography>Loading...</Typography>;
  }

  return (
    <>
      <Head>
        <title>{pool.name} - Pool Details</title>
        <meta name="description" content={`View details of ${pool.name} pool on Dexter Swap`} />
      </Head>
      <Container className={classes.root}>
        <Typography variant="h4" gutterBottom>{pool.name}</Typography>
        <Typography variant="body1">Token Pair: {pool.tokenPair}</Typography>
        <Typography variant="body1">Total Liquidity: {pool.totalLiquidity}</Typography>
        <Typography variant="body1">Transactions: {pool.transactions.length}</Typography>
        <Grid container spacing={3}>
          {pool.transactions.map((tx) => (
            <Grid item xs={12} key={tx.id}>
              <Card className={classes.card}>
                <CardContent>
                  <Typography variant="body2">Transaction ID: {tx.id}</Typography>
                  <Typography variant="body2">Type: {tx.type}</Typography>
                  <Typography variant="body2">Amount: {tx.amount}</Typography>
                  <Typography variant="body2">Timestamp: {new Date(tx.timestamp).toLocaleString()}</Typography>
                </CardContent>
              </Card>
            </Grid>
          ))}
        </Grid>
      </Container>
    </>
  );
};

export default PoolDetailsPage;
