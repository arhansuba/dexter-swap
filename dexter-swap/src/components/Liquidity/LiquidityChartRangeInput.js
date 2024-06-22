import React, { useState } from 'react';
import {
  Grid,
  Typography,
  Slider,
  makeStyles,
  Button,
  CircularProgress,
} from '@material-ui/core';

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
    padding: theme.spacing(2),
  },
  slider: {
    width: '100%',
    marginTop: theme.spacing(2),
    marginBottom: theme.spacing(2),
  },
  button: {
    marginTop: theme.spacing(2),
    height: '100%',
  },
}));

const LiquidtChartrangeInput = ({ defaultRange, onChange, loading }) => {
  const classes = useStyles();
  const [range, setRange] = useState(defaultRange);

  const handleChange = (_, newValue) => {
    setRange(newValue);
  };

  const handleApply = () => {
    if (onChange) {
      onChange(range);
    }
  };

  return (
    <div className={classes.root}>
      <Grid container spacing={3}>
        <Grid item xs={12} md={6}>
          <Typography variant="h6" gutterBottom>
            Select Range
          </Typography>
          <Slider
            value={range}
            onChange={handleChange}
            valueLabelDisplay="auto"
            aria-labelledby="range-slider"
            className={classes.slider}
          />
        </Grid>
        <Grid item xs={12} md={6}>
          <Button
            variant="contained"
            color="primary"
            disabled={loading}
            onClick={handleApply}
            className={classes.button}
          >
            {loading ? <CircularProgress size={24} color="inherit" /> : 'Apply'}
          </Button>
        </Grid>
      </Grid>
    </div>
  );
};

export default LiquidtChartrangeInput;
