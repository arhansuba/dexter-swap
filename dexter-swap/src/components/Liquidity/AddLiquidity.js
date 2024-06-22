import React, { useState } from 'react';
import {
  Grid,
  Typography,
  TextField,
  Button,
  makeStyles,
  CircularProgress,
} from '@material-ui/core';
import { useWeb3React } from '@web3-react/core'; // Replace with your preferred Web3 library
import { BigNumber } from 'ethers'; // Replace with your preferred Ethereum library for BigNumber handling

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
    padding: theme.spacing(2),
  },
  form: {
    margin: 'auto',
    maxWidth: 600,
    padding: theme.spacing(3),
    border: `1px solid ${theme.palette.divider}`,
    borderRadius: theme.shape.borderRadius,
  },
  textField: {
    marginTop: theme.spacing(2),
    marginBottom: theme.spacing(2),
  },
  button: {
    marginTop: theme.spacing(2),
    height: '100%',
  },
}));

const AddLiquidty = () => {
  const classes = useStyles();
  const { account, library } = useWeb3React(); // Web3React context for account and library

  const [tokenAmount, setTokenAmount] = useState('');
  const [ethAmount, setEthAmount] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleAddLiquidity = async () => {
    setError('');
    setLoading(true);

    try {
      // Perform validation checks here (e.g., check if amounts are valid)
      if (!account) {
        throw new Error('Please connect your wallet');
      }

      // Perform transaction to add liquidity
      const params = {
        tokenAmount: BigNumber.from(tokenAmount), // Adjust as per your token format
        ethAmount: BigNumber.from(ethAmount), // Adjust as per your ETH format
      };

      // Replace with your contract and method for adding liquidity
      const tx = await yourLiquidityPoolContract.addLiquidity(params);
      await tx.wait(); // Wait for transaction to be mined

      // Show success message or navigate to success page
      console.log('Liquidity added successfully');
    } catch (err) {
      setError(err.message || 'Failed to add liquidity');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className={classes.root}>
      <Grid container justify="center">
        <Grid item xs={12} className={classes.form}>
          <Typography variant="h5" gutterBottom>
            Add Liquidity
          </Typography>
          <TextField
            fullWidth
            variant="outlined"
            label="Token Amount"
            value={tokenAmount}
            onChange={(e) => setTokenAmount(e.target.value)}
            className={classes.textField}
          />
          <TextField
            fullWidth
            variant="outlined"
            label="ETH Amount"
            value={ethAmount}
            onChange={(e) => setEthAmount(e.target.value)}
            className={classes.textField}
          />
          {error && (
            <Typography color="error" gutterBottom>
              {error}
            </Typography>
          )}
          <Button
            fullWidth
            variant="contained"
            color="primary"
            disabled={!tokenAmount || !ethAmount || loading}
            onClick={handleAddLiquidity}
            className={classes.button}
          >
            {loading ? <CircularProgress size={24} color="inherit" /> : 'Add Liquidity'}
          </Button>
        </Grid>
      </Grid>
    </div>
  );
};

export default AddLiquidty;
