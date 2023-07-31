import { useState } from 'react';
import { ButtonEl, AleoBtnCheck } from '../shared/input';
import { usePoemsContext } from '../hooks/usePoemContext';

export default function Aleo() {
  const { steps, setSteps, setActiveStep } = usePoemsContext();
  const [showSelectFunc, setShowSelectFunc] = useState(false);

  const newSteps = steps.map((step, _) => {
    if (step.type === 'aleo') {
      step.isChecked = true;
    }
    return step;
  });
  const handleNext = () => {
    setSteps([...newSteps]);
    setActiveStep('return');
  };

  return (
    <div className="mt-4">
      <div className="w-full text-center my-8">
        <h1 className="text-aleo-dark font-bold text-3xl mb-1">
          Blind Asset Record
        </h1>
        <h1 className="text-sm w-[90%] mx-auto">
          Encrypt sign and send your function hashes to Aleo
        </h1>
        <div className="flex mt-5 flex-wrap gap-y-2 justify-around items-center mb-8">
          <AleoBtnCheck id={'purchase'} text={'Purchase'} />
          <AleoBtnCheck id={'recover-account'} text={'Recover Account'} />
          <AleoBtnCheck id={'spend'} text={'Spend'} />
          <AleoBtnCheck id={'transfer'} text={'Transfer'} />
          <AleoBtnCheck id={'send'} text={'Send'} />
        </div>
      </div>
      <div className="flex flex-col md:flex-row justify-between items-center"></div>
      <div className="mb-8 w-[80%] m-auto"></div>
      <div className="w-full mb-8 flex justify-center items-center">
        <ButtonEl
          handleClick={() => setShowSelectFunc(!showSelectFunc)}
          className="border border-white bg-aleo-dark hover:bg-aleo-light text-white"
          text="Encrypt functions"
        />
      </div>
      {showSelectFunc && (
        <div>
          <div>
            <div className="w-full text-center my-8">
              <h1 className="text-aleo-dark font-bold text-3xl mb-1">
                Nucleo Wallet
              </h1>
              <h1 className="text-sm w-[90%] mx-auto">
                You agree to sign and send your Blind Asset Record from Poems to
                Aleo
              </h1>
            </div>
            <div className="mb-8 w-[80%] m-auto">
              <div class="flex items-center mb-4">
                <input
                  id="default-radio-1"
                  type="radio"
                  value=""
                  name="default-radio"
                  className="w-4 h-4 text-white focus:ring-white"
                />
                <label
                  htmlFor="default-radio-1"
                  className="ml-2 text-sm font-medium text-aleo-light"
                >
                  I agree
                </label>
              </div>
              <div class="flex items-center">
                <input
                  checked
                  id="default-radio-2"
                  type="radio"
                  value=""
                  name="default-radio"
                  className="w-4 h-4 text-white focus:ring-white"
                />
                <label
                  htmlFor="default-radio-2"
                  className="ml-2 text-sm font-medium text-aleo-light"
                >
                  I do not agree to send Aleo my encrypted data
                </label>
              </div>
            </div>
          </div>
          <div className="w-full mb-8 flex justify-center items-center">
            <ButtonEl
              handleClick={handleNext}
              className="border border-white bg-aleo-dark hover:bg-aleo-light text-white"
              text="Send"
            />
          </div>
        </div>
      )}
    </div>
  );
}
