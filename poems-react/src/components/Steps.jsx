import { usePoemsContext } from '../hooks/usePoemContext';
<<<<<<< HEAD
import { CgCheckO, CgRadioCheck, CgArrowLeft } from 'react-icons/cg'; 
import {PrevButton} from '../shared/input/index'
import {useState, useEffect} from 'react'



=======
<<<<<<< HEAD
import { CgCheckO, CgRadioCheck } from 'react-icons/cg';
>>>>>>> 17ad3de (Add step icons and labels to Steps component)

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
<<<<<<< HEAD
=======
=======
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
>>>>>>> 17ad3de (Add step icons and labels to Steps component)

      {
         activeStep  !== 'account'  && activeStep !== 'return' && activeStep !== 'decrypt' ?   <PrevButton steps={steps}/> : ''
      }

<<<<<<< HEAD
=======
>>>>>>> 93a5493 (Add step icons and labels to Steps component)
>>>>>>> 17ad3de (Add step icons and labels to Steps component)
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
