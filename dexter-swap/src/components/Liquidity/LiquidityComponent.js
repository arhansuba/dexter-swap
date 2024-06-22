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
import { addLiquidity, removeLiquidity } from '../../utils/xionUtil'; // Replace with actual utility functions

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
  button: {
    marginTop: theme.spacing(2),
  },
}));

const LiquidityComponent = ({ tokens }) => {
  const classes = useStyles();

  const [token, setToken] = useState('');
  const [amount, setAmount] = useState('');
  const [action, setAction] = useState('add'); // 'add' or 'remove'

  const handleActionChange = (event) => {
    setAction(event.target.value);
  };

  const handleTokenChange = (event) => {
    setToken(event.target.value);
  };

  const handleAmountChange = (event) => {
    setAmount(event.target.value);
  };

  const handleSubmit = async () => {
    if (action === 'add') {
      await addLiquidity(token, amount);
    } else {
      await removeLiquidity(token, amount);
    }
    // Reset form after action
    setToken('');
    setAmount('');
  };

  return (
    <Grid container spacing={3} className={classes.root}>
      <Grid item xs={12}>
        <Typography variant="h5">{action === 'add' ? 'Add Liquidity' : 'Remove Liquidity'}</Typography>
      </Grid>
      <Grid item xs={12} sm={6}>
        <FormControl fullWidth className={classes.formControl}>
          <InputLabel id="action-label">Action</InputLabel>
          <Select
            labelId="action-label"
            id="action-select"
            value={action}
            onChange={handleActionChange}
          >
            <MenuItem value="add">Add Liquidity</MenuItem>
            <MenuItem value="remove">Remove Liquidity</MenuItem>
          </Select>
        </FormControl>
      </Grid>
      <Grid item xs={12} sm={6}>
        <FormControl fullWidth className={classes.formControl}>
          <InputLabel id="token-label">Token</InputLabel>
          <Select
            labelId="token-label"
            id="token-select"
            value={token}
            onChange={handleTokenChange}
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
        <TextField
          id="amount"
          label="Amount"
          type="number"
          fullWidth
          className={classes.inputField}
          value={amount}
          onChange={handleAmountChange}
        />
      </Grid>
      <Grid item xs={12}>
        <Button
          variant="contained"
          color="primary"
          onClick={handleSubmit}
          className={classes.button}
        >
          {action === 'add' ? 'Add Liquidity' : 'Remove Liquidity'}
        </Button>
      </Grid>
    </Grid>
  );
};

export default LiquidityComponent;
