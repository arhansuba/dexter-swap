import React, { useState } from 'react';
import {
  TextField,
  Button,
  Grid,
  Typography,
  makeStyles,
} from '@material-ui/core';
import { isValidAddress } from 'ethereumjs-util'; // Replace with your preferred library for Ethereum address validation

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
    padding: theme.spacing(2),
  },
  textField: {
    marginRight: theme.spacing(2),
  },
  button: {
    height: '100%',
  },
}));

const AddressInputPanel = ({ onAddressSubmit }) => {
  const classes = useStyles();
  const [address, setAddress] = useState('');
  const [error, setError] = useState('');

  const handleChange = (event) => {
    setAddress(event.target.value);
    setError('');
  };

  const handleSubmit = () => {
    if (!isValidAddress(address)) {
      setError('Invalid Ethereum address');
      return;
    }

    // Pass the validated address to the parent component
    onAddressSubmit(address);
    setAddress('');
  };

  return (
    <Grid container className={classes.root} spacing={2}>
      <Grid item xs={8}>
        <TextField
          fullWidth
          variant="outlined"
          label="Ethereum Address"
          value={address}
          onChange={handleChange}
          error={!!error}
          helperText={error}
          className={classes.textField}
        />
      </Grid>
      <Grid item xs={4}>
        <Button
          fullWidth
          variant="contained"
          color="primary"
          onClick={handleSubmit}
          className={classes.button}
        >
          Submit
        </Button>
      </Grid>
    </Grid>
  );
};

export default AddressInputPanel;
