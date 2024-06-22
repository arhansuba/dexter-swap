import React from 'react';
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
  Typography,
  makeStyles,
} from '@material-ui/core';

const useStyles = makeStyles((theme) => ({
  dialogTitle: {
    backgroundColor: theme.palette.primary.main,
    color: theme.palette.common.white,
  },
  dialogContent: {
    marginTop: theme.spacing(2),
  },
  dialogActions: {
    justifyContent: 'space-between',
    padding: theme.spacing(2),
  },
}));

const ConfirmSwapModal = ({ open, onClose, onConfirm, swapDetails }) => {
  const classes = useStyles();

  const handleConfirm = () => {
    if (onConfirm) {
      onConfirm();
    }
  };

  return (
    <Dialog open={open} onClose={onClose}>
      <DialogTitle className={classes.dialogTitle}>Confirm Swap</DialogTitle>
      <DialogContent className={classes.dialogContent}>
        <Typography variant="subtitle1">
          Please confirm the following swap details:
        </Typography>
        <Typography variant="body1">
          From: {swapDetails.fromToken} {swapDetails.fromAmount}
        </Typography>
        <Typography variant="body1">
          To: {swapDetails.toToken} {swapDetails.toAmount}
        </Typography>
      </DialogContent>
      <DialogActions className={classes.dialogActions}>
        <Button onClick={onClose} color="secondary" variant="outlined">
          Cancel
        </Button>
        <Button onClick={handleConfirm} color="primary" variant="contained">
          Confirm Swap
        </Button>
      </DialogActions>
    </Dialog>
  );
};

export default ConfirmSwapModal;
