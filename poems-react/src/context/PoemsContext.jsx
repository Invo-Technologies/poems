import React from 'react';
import PropTypes from 'prop-types';
import usePersist from '../hooks/usePersist';

export const PoemsContext = React.createContext({});

export const PoemsContextProvider = ({ children }) => {
  const [activeStep, setActiveStep] = usePersist('activeStep', 'account');
  const [steps, setSteps] = usePersist('steps', [
    {
      type: 'account',
      isChecked: false,
    },
    {
      type: 'generation',
      isChecked: false,
    },
    {
      type: 'aleo',
      isChecked: false,
    },
    {
      type: 'return',
      isChecked: false,
    },
    {
      type: 'decrypt',
      isChecked: false,
    },
  ]);
  return (
    <PoemsContext.Provider
      value={{ activeStep, setActiveStep, steps, setSteps }}
    >
      {children}
    </PoemsContext.Provider>
  );
};

PoemsContextProvider.propTypes = {
  children: PropTypes.node.isRequired,
};
