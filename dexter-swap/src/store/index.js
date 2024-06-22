import { createStore, applyMiddleware, compose } from 'redux';
import thunk from 'redux-thunk'; // Example: Importing redux-thunk for async actions
import { createBrowserHistory } from 'history'; // Example: Importing history for connected-react-router
import { routerMiddleware } from 'connected-react-router'; // Example: Importing router middleware for connected-react-router
import createRootReducer from '../reducers'; // Importing your root reducer from reducers/index.js

// Create a history object for navigation in connected-react-router
export const history = createBrowserHistory();

// Middleware setup
const middleware = [thunk, routerMiddleware(history)]; // Example: Including redux-thunk and router middleware

// Redux DevTools Extension setup
const composeEnhancers = window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose;

// Create the Redux store instance
const store = createStore(
  createRootReducer(history), // Pass the root reducer with history to manage router state
  composeEnhancers(applyMiddleware(...middleware)) // Apply middleware and enhancers
);

export default store;
