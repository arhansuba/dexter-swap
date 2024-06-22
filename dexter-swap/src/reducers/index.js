import { combineReducers } from 'redux';
import { connectRouter } from 'connected-react-router'; // Import if using connected-react-router
import { reducer as formReducer } from 'redux-form'; // Import if using redux-form
import accountReducer from './accountReducer'; // Import your specific reducers here

// Root reducer function
const createRootReducer = (history) =>
  combineReducers({
    router: connectRouter(history), // Connect router if using connected-react-router
    form: formReducer, // Add form reducer if using redux-form
    account: accountReducer, // Example: Your account reducer
    // Add more reducers as needed
  });

export default createRootReducer;
