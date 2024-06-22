import React, { useState } from 'react';
import {
  Grid,
  Typography,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  TextField,
  Button,
  makeStyles,
} from '@material-ui/core';

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

  const [fromToken, setFromToken] = useState('');
  const [toToken, setToToken] = useState('');
  const [fromAmount, setFromAmount] = useState('');
  const [toAmount, setToAmount] = useState('');

  const handleSwap = () => {
    // Simulate swap logic, replace with actual implementation
    console.log(`Swapping ${fromAmount} ${fromToken} for ${toAmount} ${toToken}`);
    // Reset form after swap
    setFromToken('');
    setToToken('');
    setFromAmount('');
    setToAmount('');
  };

  const handleChangeFromToken = (event) => {
    setFromToken(event.target.value);
  };

  const handleChangeToToken = (event) => {
    setToToken(event.target.value);
  };

  const handleChangeFromAmount = (event) => {
    setFromAmount(event.target.value);
  };

  return (
    <Grid container spacing={3} className={classes.root}>
      <Grid item xs={12}>
        <Typography variant="h5">Swap Tokens</Typography>
      </Grid>
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
          id="from-amount"
          label="Amount"
          type="number"
          fullWidth
          className={classes.inputField}
          value={fromAmount}
          onChange={handleChangeFromAmount}
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
        <TextField
          id="to-amount"
          label="Amount"
          type="number"
          fullWidth
          className={classes.inputField}
          value={toAmount}
          InputProps={{ readOnly: true }}
        />
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
  );
};

export default SwapComponent;
