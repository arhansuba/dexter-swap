import React, { useState } from 'react';
import {
  Grid,
  Typography,
  TextField,
  Button,
  CircularProgress,
  makeStyles,
} from '@material-ui/core';

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
    padding: theme.spacing(2),
  },
  inputField: {
    marginBottom: theme.spacing(2),
  },
  button: {
    marginTop: theme.spacing(2),
  },
}));

const RemoveLiquidity = ({ onRemove, loading }) => {
  const classes = useStyles();
  const [amount, setAmount] = useState('');

  const handleAmountChange = (event) => {
    setAmount(event.target.value);
  };

  const handleRemove = () => {
    if (onRemove) {
      onRemove(amount);
    }
  };

  return (
    <div className={classes.root}>
      <Grid container spacing={3}>
        <Grid item xs={12}>
          <Typography variant="h6" gutterBottom>
            Remove Liquidity
          </Typography>
        </Grid>
        <Grid item xs={12}>
          <TextField
            id="amount"
            label="Amount"
            type="number"
            variant="outlined"
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
            disabled={!amount || loading}
            onClick={handleRemove}
            className={classes.button}
          >
            {loading ? <CircularProgress size={24} color="inherit" /> : 'Remove'}
          </Button>
        </Grid>
      </Grid>
    </div>
  );
};

export default RemoveLiquidity;
