import { usePoemsContext } from '../hooks/usePoemContext';
import { CgCheckO, CgRadioCheck, CgArrowLeft } from 'react-icons/cg'; 
import {PrevButton} from '../shared/input/index'
import {useState, useEffect} from 'react'




export default function Steps() {
  const { steps } = usePoemsContext();
    const { activeStep } = usePoemsContext();
      // console.log(activeStep)
      const { stepss,  setSteps, setActiveStep } = usePoemsContext();
      // const goback = (step) =>{

      //   if(activeStep!== 'return'){

      //       setActiveStep(step.type);
      //       step.isChecked = false
      //     }  
      // }

  return (
    <div className="flex justify-around my-4 items-center">

      {
         activeStep  !== 'account'  && activeStep !== 'return' && activeStep !== 'decrypt' ?   <PrevButton steps={steps}/> : ''
      }

      {steps.map((step, idx) => {
        const { type, isChecked } = step;
        return (
          <div key={idx}>
            {isChecked ? (
              <div
                className={`
                flex flex-col justify-center items-center
                ${
                  type === 'account'
                    ? `text-account-dark`
                    : type === 'generation'
                    ? `text-generation-dark`
                    : type === 'aleo'
                    ? `text-aleo-dark`
                    : type === 'return'
                    ? `text-return-dark`
                    : type === 'decrypt'
                    ? `text-decrypt-dark`
                    : ''
                }  
            `}
              >
                <CgCheckO className={`text-4xl`} />
                <small>{type}</small>
              </div>
            ) : (
              <div className="flex flex-col justify-center items-center">
                <CgRadioCheck className={`text-4xl`} />
                <small>{type}</small>
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
