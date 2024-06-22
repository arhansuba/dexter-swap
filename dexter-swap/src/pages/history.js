import React, { useState, useEffect } from 'react';
import Head from 'next/head';
import { Container, Typography, Grid, Card, CardContent, makeStyles } from '@material-ui/core';
import { useAbstraxionAccount } from '@burnt-labs/abstraxion';
import { getUserTransactions } from '../utils/xionUtil';  // Utility function to fetch user transactions

const useStyles = makeStyles((theme) => ({
  root: {
    padding: theme.spacing(3),
  },
  card: {
    marginBottom: theme.spacing(2),
  },
}));

const HistoryPage = () => {
  const classes = useStyles();
  const { account } = useAbstraxionAccount();
  const [transactions, setTransactions] = useState([]);

  useEffect(() => {
    if (account) {
      const fetchTransactions = async () => {
        const data = await getUserTransactions(account.address);
        setTransactions(data);
      };
      fetchTransactions();
    }
  }, [account]);

  if (!transactions.length) {
    return <Typography>Loading...</Typography>;
  }

  return (
    <>
      <Head>
        <title>Transaction History - Dexter Swap</title>
        <meta name="description" content="View your transaction history on Dexter Swap" />
      </Head>
      <Container className={classes.root}>
        <Typography variant="h4" gutterBottom>Transaction History</Typography>
        <Grid container spacing={3}>
          {transactions.map((tx) => (
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

export default HistoryPage;
