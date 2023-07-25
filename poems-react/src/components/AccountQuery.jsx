import { useState } from "react";
import { InputEl, ButtonEl } from "../shared/input";
import { usePoemsContext } from "../hooks/usePoemContext";

export default function AccountQuery() {
  const [showAcctID, setShowAcctID] = useState(false);
  const { steps, setSteps, setActiveStep } = usePoemsContext();
  const newSteps = steps.map((step, _) => {
    if (step.type === "account") {
      step.isChecked = true;
    }
    return step;
  });
  const handleNext = () => {
    setSteps([...newSteps]);
    setActiveStep("generation");
  };
  return (
    <div className="mt-4">
      <div className="w-full text-center my-8">
        <h1 className="text-account-dark font-bold text-3xl mb-1">
          Account Query
        </h1>
        <h1 className="text-sm w-[90%] mx-auto">
          Once you have assigned a name to the default currency field, proceed
          to generate your Account Query please
        </h1>
      </div>
      <div className="flex flex-col md:flex-row justify-between items-center">
        <div className="w-full md:w-[45%]">
          <InputEl
            className="bg-white text-black cursor-pointer"
            showLabel={true}
            labelText="Default currency"
            placeholder="currency:"
          />
        </div>
        <div className="w-full md:w-[45%]">
          <InputEl
            className="bg-white text-black cursor-pointer"
            showLabel={true}
            disabled={true}
            labelText="Asset ID"
            placeholder="asset ID:"
          />
        </div>
      </div>
      <div className="flex flex-col md:flex-row justify-between items-center">
        <div className="w-full md:w-[45%]">
          <InputEl
            className="bg-white text-black cursor-pointer"
            showLabel={true}
            labelText="Gamer Tag"
            placeholder="gamer tag:"
          />
        </div>
        <div className="w-full md:w-[45%]">
          <InputEl
            showLabel={false}
            disabled={true}
            placeholder="Account ID:"
          />
        </div>
      </div>

      <div className="flex flex-col justify-between items-center">
        <InputEl
          className="bg-white text-black cursor-pointer"
          showLabel={true}
          disabled={true}
          labelText="Node ID"
          placeholder="node id"
        />
        <InputEl
          className="bg-white text-black cursor-pointer"
          showLabel={true}
          disabled={true}
          labelText="Game ID"
          placeholder="game id"
        />
        <InputEl
          className="bg-white text-black cursor-pointer"
          showLabel={true}
          disabled={true}
          labelText="Pool ID"
          placeholder="pool id"
        />
      </div>
      <div className="w-full flex justify-end">
        <ButtonEl
          handleClick={() => setShowAcctID(!showAcctID)}
          className="border border-white bg-account-dark hover:bg-account-light text-white"
          text="Create Account query"
        />
      </div>
      {showAcctID && (
        <div className="pt-5 ">
          <div className="mt-1">
            <div className="text-center">
              In the realm where coded secrets dwell,
            </div>
            <div className="mt-4 text-center">
              Your interpretations weave a spell,
            </div>
            <div className="mt-4 text-center">
              Store them easily for they are the key,
            </div>
            <div className="mt-4 text-center">
              To decrypt functions, you'll soon see,
            </div>
          </div>
          <div className="w-full mb-8 flex justify-end">
            <ButtonEl
              handleClick={handleNext}
              className="border border-white bg-account-light hover:bg-account-dark hover:text-white text-account-dark"
              text="Next"
            />
          </div>
        </div>
      )}
    </div>
  );
}
