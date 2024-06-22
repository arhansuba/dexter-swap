import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import { makeStyles, CssBaseline, Container } from '@material-ui/core';
import Header from '../components/Header';
import Home from '../pages/Home';
import Trade from '../pages/Trade';
import Liquidity from '../pages/Liquidity';
import About from '../pages/About';
import Footer from '../components/Footer';

const useStyles = makeStyles((theme) => ({
  root: {
    display: 'flex',
    flexDirection: 'column',
    minHeight: '100vh',
  },
  main: {
    marginTop: theme.spacing(8),
    marginBottom: theme.spacing(2),
  },
}));

const App = () => {
  const classes = useStyles();

  return (
    <Router>
      <div className={classes.root}>
        <CssBaseline />
        <Header />
        <Container component="main" className={classes.main}>
          <Switch>
            <Route exact path="/" component={Home} />
            <Route path="/trade" component={Trade} />
            <Route path="/liquidity" component={Liquidity} />
            <Route path="/about" component={About} />
            {/* Add more routes as needed */}
          </Switch>
        </Container>
        <Footer />
      </div>
    </Router>
  );
};

export default App;
