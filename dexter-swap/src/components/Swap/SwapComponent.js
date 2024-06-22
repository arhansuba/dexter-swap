// src/components/Swap/SwapComponent.js
import React, { useState } from 'react';
import { Grid, Typography, FormControl, InputLabel, Select, MenuItem, TextField, Button, makeStyles } from '@material-ui/core';
import { connectWallet, executeTradingAction } from "../../utils/xionUtil";
import { Input } from "@burnt-labs/ui";

const useStyles = makeStyles((theme) => ({
  root: {
    padding: theme.spacing(3),
  },
  formControl: {
    minWidth: 200,
    marginBottom: theme.spacing(2),
  },
  inputField: {
    marginBottom: theme.spacing(2),
  },
  swapButton: {
    marginTop: theme.spacing(2),
  },
}));

const SwapComponent = ({ tokens }) => {
  const classes = useStyles();

  const [mnemonic, setMnemonic] = useState("");
  const [address, setAddress] = useState("");
  const [fromToken, setFromToken] = useState('');
  const [toToken, setToToken] = useState('');
  const [amount, setAmount] = useState('');
  const [swapDetails, setSwapDetails] = useState({ from: "", to: "", amount: 0 });

  const handleConnect = async () => {
    await connectWallet(mnemonic);
    setAddress(mnemonic); // This assumes the mnemonic serves as the address, replace with actual address fetch logic
  };

  const handleSwap = async () => {
    const msg = {
      swap: {
        from: swapDetails.from,
        to: swapDetails.to,
        amount: swapDetails.amount,
      },
    };
    const result = await executeTradingAction(msg);
    console.log(result);
  };

  const handleChangeFromToken = (event) => {
    setFromToken(event.target.value);
    setSwapDetails({ ...swapDetails, from: event.target.value });
  };

  const handleChangeToToken = (event) => {
    setToToken(event.target.value);
    setSwapDetails({ ...swapDetails, to: event.target.value });
  };

  const handleChangeAmount = (event) => {
    setAmount(event.target.value);
    setSwapDetails({ ...swapDetails, amount: event.target.value });
  };

  return (
    <div className={classes.root}>
      <Typography variant="h5">Swap Tokens</Typography>
      <Input placeholder="Mnemonic" onChange={(e) => setMnemonic(e.target.value)} />
      <Button onClick={handleConnect}>Connect Wallet</Button>
      {address && <Typography variant="body1">Connected as: {address}</Typography>}

      <Grid container spacing={3}>
        <Grid item xs={12} sm={6}>
          <FormControl fullWidth className={classes.formControl}>
            <InputLabel id="from-token-label">From Token</InputLabel>
            <Select
              labelId="from-token-label"
              id="from-token-select"
              value={fromToken}
              onChange={handleChangeFromToken}
            >
              {tokens.map((token) => (
                <MenuItem key={token} value={token}>
                  {token}
                </MenuItem>
              ))}
            </Select>
          </FormControl>
          <TextField
            id="amount"
            label="Amount"
            type="number"
            fullWidth
            className={classes.inputField}
            value={amount}
            onChange={handleChangeAmount}
          />
        </Grid>
        <Grid item xs={12} sm={6}>
          <FormControl fullWidth className={classes.formControl}>
            <InputLabel id="to-token-label">To Token</InputLabel>
            <Select
              labelId="to-token-label"
              id="to-token-select"
              value={toToken}
              onChange={handleChangeToToken}
            >
              {tokens.map((token) => (
                <MenuItem key={token} value={token}>
                  {token}
                </MenuItem>
              ))}
            </Select>
          </FormControl>
        </Grid>
        <Grid item xs={12}>
          <Button
            variant="contained"
            color="primary"
            onClick={handleSwap}
            className={classes.swapButton}
          >
            Swap
          </Button>
        </Grid>
      </Grid>
    </div>
  );
};

export default SwapComponent;
