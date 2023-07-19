import { useContext } from 'react';
import { PoemsContext } from '../context/poemsContext';

export const usePoemsContext = () => {
  return useContext(PoemsContext);
};
