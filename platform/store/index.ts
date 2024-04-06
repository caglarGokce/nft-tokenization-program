import { configureStore } from '@reduxjs/toolkit';
import Reducer from './reducers';

/** Redux store for the application */
const store = configureStore({ reducer: Reducer });

export default store;
