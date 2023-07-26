import { useState } from "react";
import { InputEl, ButtonEl, DecryptBtnCheck } from "../shared/input";
import { usePoemsContext } from "../hooks/usePoemContext";

export default function Decrypt() {
  const { steps, setSteps, setActiveStep } = usePoemsContext();

  const newSteps = steps.map((step, _) => {
    if (step.type === "decrypt") {
      step.isChecked = true;
    }
    return step;
  });

  const [success, setSuccess] = useState(false)
  const [successMessage, setSuccessMessage] = useState(false)

  const handleNext = () => {
    setSteps([...newSteps]);
    setActiveStep("decrypt");
    setSuccess(true)
    
    setTimeout(() => {
      let getLoad = document.getElementById('load')
      getLoad.style.display = 'none'
      setSuccessMessage(true)
    }, 7500)
  };


    
    
  


  return (
    <div className="mt-4">
      <div className="w-full text-center my-8">
        <h1 className="text-sm w-[90%] mx-auto">
          Poems has removed your mnemonic from memory and does not share the
          same interpretations as you do.
        </h1>
      </div>

      <div className="w-[90%] max-w-xl mx-auto">
        <div className="flex flex-wrap gap-y-2 justify-around items-center mb-8">
          <DecryptBtnCheck id={"purchase"} text={"Purchase"} />
          <DecryptBtnCheck id={"recover-account"} text={"Recover Account"} />
          <DecryptBtnCheck id={"spend"} text={"Spend"} />
          <DecryptBtnCheck id={"transfer"} text={"Transfer"} />
          <DecryptBtnCheck id={"send"} text={"Send"} />
        </div>

        <div className="w-full max-w-lg mx-auto">
          <ol className="text-decrypt-light list-decimal mb-8">
            <li>
              Open your saved poem-interpretations file saved in your browsers
              downloads folder
            </li>
            <li>
              Select the Function ID you would like to use and paste the
              interpretation's in X and S Keys in the correct filed
            </li>
            <li>Click "Decrypt" and the program will interprete your poem.</li>
          </ol>
        </div>

        <div className="mb-8 m-auto">
          <InputEl
            showLabel={false}
            labelText="x"
            className="invisible"
            placeholder="X:"
          />

          <InputEl
            showLabel={false}
            labelText="s"
            className="invisible"
            placeholder="S:"
          />
        </div>
      </div>

      <div className="w-full mb-8 flex justify-center items-center">
        <ButtonEl
          handleClick={handleNext}
          className="border border-white bg-decrypt-dark hover:bg-decrypt-light text-white"
          text="Decrypt functions"
        />

        <div className='ml-5' style={{ display: success ? 'flex' : 'none' }} id='load'>
          <button href='' className='btn_decrypt' type=''>
            <span></span>
            <span></span>
            <span></span>
            <span></span>
            decrypting <i className='bi-arrow-right'></i>
          </button>
        </div>

        <div className='success ml-2' style={{ display: successMessage ? 'flex' : 'none'  }}>
          Success!
        </div>
      </div>

      
    </div>
  );
}
