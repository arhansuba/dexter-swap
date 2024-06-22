import React from 'react';
import { makeStyles, Typography, Link } from '@material-ui/core';

const useStyles = makeStyles((theme) => ({
  root: {
    backgroundColor: '#f0f0f0',
    padding: theme.spacing(3),
    marginTop: 'auto',
    textAlign: 'center',
  },
  link: {
    margin: theme.spacing(0, 2),
    textDecoration: 'none',
    color: theme.palette.text.primary,
    '&:hover': {
      textDecoration: 'underline',
    },
  },
}));

const Footer = () => {
  const classes = useStyles();

  return (
    <div className={classes.root}>
      <Typography variant="body2" color="textSecondary">
        &copy; {new Date().getFullYear()} Dexter Swap. All rights reserved.
      </Typography>
      <Typography variant="body2" color="textSecondary">
        Powered by{' '}
        <Link
          href="https://ethereum.org/"
          target="_blank"
          rel="noopener noreferrer"
          className={classes.link}
        >
          Ethereum
        </Link>
        {' | '}
        <Link
          href="https://reactjs.org/"
          target="_blank"
          rel="noopener noreferrer"
          className={classes.link}
        >
          React
        </Link>
        {' | '}
        <Link
          href="https://material-ui.com/"
          target="_blank"
          rel="noopener noreferrer"
          className={classes.link}
        >
          Material-UI
        </Link>
      </Typography>
    </div>
  );
};

export default Footer;
