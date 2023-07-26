import { useState } from 'react';
import { InputEl, ButtonEl, GenerationBtnCheck } from '../shared/input';
import { usePoemsContext } from '../hooks/usePoemContext';

export default function Generation() {
  const [showSelectFunc, setShowSelectFunc] = useState(false);
  const { steps, setSteps, setActiveStep } = usePoemsContext();

  const newSteps = steps.map((step, _) => {
    if (step.type === 'generation') {
      step.isChecked = true;
    }
    return step;
  });

  const handleNext = () => {
    setSteps([...newSteps]);
    setActiveStep('aleo');
  };

  return (
    <div className="mt-4">
      <div className="w-full text-center my-8">
        <h1 className="text-generation-dark font-bold text-3xl mb-1">
          Key Generation
        </h1>
        <h1 className="text-sm w-[90%] mx-auto">
          Create your keys to encode a new secret
        </h1>
      </div>
      <div className="flex flex-col md:flex-row justify-between items-center">
        <div className="w-full md:w-[45%]">
          <InputEl
            className="bg-black text-generation-dark cursor-pointer"
            showLabel={true}
            labelText="BEP 39"
            disabled={true}
            placeholder="Mnemonic:"
          />
        </div>

        <div className="w-full md:w-[45%]">
          <InputEl
            showLabel={true}
            labelText="seed"
            className="invisible"
            disabled={true}
            placeholder="seed:"
          />
        </div>
      </div>

      <div className="flex flex-col md:flex-row justify-between items-center">
        <div className="w-full md:w-[45%]">
          <InputEl
            className="bg-black text-generation-dark cursor-pointer"
            showLabel={true}
            labelText="RSA"
            disabled={true}
            placeholder="Public key:"
          />
        </div>

        <div className="w-full md:w-[45%]">
          <InputEl
            showLabel={true}
            labelText="seed"
            className="invisible"
            disabled={true}
            placeholder="private key:"
          />
        </div>
      </div>

      <div className="flex flex-col md:flex-row justify-between items-center">
        <div className="w-full md:w-[45%]">
          <InputEl
            className="bg-black text-generation-dark cursor-pointer"
            showLabel={true}
            labelText="seed"
            disabled={true}
            placeholder="Public key:"
          />
        </div>

        <div className="w-full md:w-[45%]">
          <InputEl
            showLabel={true}
            labelText="seed"
            disabled={true}
            className="invisible"
            placeholder="private key:"
          />
        </div>
      </div>

      <div className="w-full flex justify-end">
        <ButtonEl
          handleClick={() => setShowSelectFunc(!showSelectFunc)}
          className="border border-white bg-generation-dark hover:bg-generation-light text-white"
          text="Generate Keys"
        />
      </div>

      {showSelectFunc && (
        <div>
          <div className="w-full text-center my-8">
            <h1 className="text-generation-dark font-bold text-3xl mb-1">
              Select Functions
            </h1>
            <h1 className="text-sm w-[90%] mx-auto">
              Keys unlock the gameplay experience. Select the features you wish
              to incoporate in your game
            </h1>
          </div>

          <div className="flex flex-wrap gap-y-2 justify-around items-center mb-8">
            <GenerationBtnCheck id={'purchase'} text={'Purchase'} />
            <GenerationBtnCheck
              id={'recover-account'}
              text={'Recover Account'}
            />
            <GenerationBtnCheck id={'spend'} text={'Spend'} />
            <GenerationBtnCheck id={'transfer'} text={'Transfer'} />
            <GenerationBtnCheck id={'send'} text={'Send'} />
          </div>

          <div className="w-full text-center my-8">
            <h1 className="text-sm w-[90%] mx-auto">
              For a comprehensive demonstartion, we encourage you to select a
              few of the functions - ideally those does not choosen during the
              decryption phase to fully immerse yourself in the demo experience
            </h1>
          </div>

          <div className="w-full mb-8 flex justify-end">
            <ButtonEl
              handleClick={handleNext}
              className="border border-white bg-generation-dark hover:bg-generation-light text-white"
              text="Next"
            />
          </div>
        </div>
      )}
    </div>
  );
}
