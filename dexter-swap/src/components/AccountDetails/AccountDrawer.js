import React, { useState, useEffect } from 'react';
import {
  Drawer,
  List,
  ListItem,
  ListItemText,
  Divider,
  Typography,
  IconButton,
} from '@material-ui/core';
import CloseIcon from '@material-ui/icons/Close';
import { ethers } from 'ethers'; // Replace with your Ethereum library
import { useEthers } from '@usedapp/core'; // Replace with your preferred Ethereum hooks library
import { formatEther } from 'ethers/lib/utils'; // Replace with your ethers utility method for formatting
import Web3 from 'web3'; // Replace with your Web3 library if not using ethers

const AccountDrawer = ({ open, onClose }) => {
  const { activateBrowserWallet, account, deactivate } = useEthers(); // Replace with your preferred hooks usage

  const connectWallet = async () => {
    try {
      await activateBrowserWallet(); // Replace with activation method for your preferred wallet
    } catch (error) {
      console.error('Failed to connect wallet:', error);
    }
  };

  const disconnectWallet = () => {
    try {
      deactivate(); // Replace with deactivation method for your preferred wallet
    } catch (error) {
      console.error('Failed to disconnect wallet:', error);
    }
  };

  return (
    <Drawer anchor="right" open={open} onClose={onClose}>
      <div>
        <div style={{ display: 'flex', justifyContent: 'space-between', padding: '16px' }}>
          <Typography variant="h6">Account</Typography>
          <IconButton onClick={onClose}>
            <CloseIcon />
          </IconButton>
        </div>
        <Divider />
        <List>
          {account ? (
            <>
              <ListItem>
                <ListItemText primary="Connected Account" secondary={account} />
              </ListItem>
              <ListItem>
                <ListItemText primary="Balance" secondary={`${formatEther(balance)} ETH`} />
              </ListItem>
              <ListItem button onClick={disconnectWallet}>
                <ListItemText primary="Disconnect Wallet" />
              </ListItem>
            </>
          ) : (
            <ListItem button onClick={connectWallet}>
              <ListItemText primary="Connect Wallet" />
            </ListItem>
          )}
        </List>
      </div>
    </Drawer>
  );
};

export default AccountDrawer;
