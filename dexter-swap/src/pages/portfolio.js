import React, { useState, useEffect } from 'react';
import Head from 'next/head';
import { Container, Typography, Grid, Card, CardContent, makeStyles } from '@material-ui/core';
import { useAbstraxionAccount } from '@burnt-labs/abstraxion';
import { getUserPortfolio } from '../utils/xionUtil';  // Utility function to fetch user portfolio

const useStyles = makeStyles((theme) => ({
  root: {
    padding: theme.spacing(3),
  },
  card: {
    marginBottom: theme.spacing(2),
  },
}));

const PortfolioPage = () => {
  const classes = useStyles();
  const { account } = useAbstraxionAccount();
  const [portfolio, setPortfolio] = useState(null);

  useEffect(() => {
    if (account) {
      const fetchPortfolio = async () => {
        const data = await getUserPortfolio(account.address);
        setPortfolio(data);
      };
      fetchPortfolio();
    }
  }, [account]);

  if (!portfolio) {
    return <Typography>Loading...</Typography>;
  }

  return (
    <>
      <Head>
        <title>Portfolio - Dexter Swap</title>
        <meta name="description" content="View your portfolio on Dexter Swap" />
      </Head>
      <Container className={classes.root}>
        <Typography variant="h4" gutterBottom>My Portfolio</Typography>
        <Grid container spacing={3}>
          {portfolio.positions.map((position) => (
            <Grid item xs={12} key={position.id}>
              <Card className={classes.card}>
                <CardContent>
                  <Typography variant="body2">Position ID: {position.id}</Typography>
                  <Typography variant="body2">Token: {position.token}</Typography>
                  <Typography variant="body2">Amount: {position.amount}</Typography>
                  <Typography variant="body2">Value: {position.value}</Typography>
                </CardContent>
              </Card>
            </Grid>
          ))}
        </Grid>
      </Container>
    </>
  );
};

export default PortfolioPage;
