import { InputEl, ButtonEl } from '../shared/input';

export default function AccountQuery() {
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
          <InputEl labelText="Default currency" placeholder="currency:" />
        </div>

        <div className="w-full md:w-[45%]">
          <InputEl
            disabled={true}
            labelText="Asset ID"
            placeholder="asset ID:"
          />
        </div>
      </div>

      <div className="flex flex-col justify-between items-center">
        <InputEl disabled={true} labelText="Node ID" placeholder="node id" />
        <InputEl disabled={true} labelText="Game ID" placeholder="game id" />
        <InputEl disabled={true} labelText="Proof ID" placeholder="proof id" />
      </div>

      <div className="w-full flex justify-end">
        <ButtonEl
          className="border border-white bg-account-dark hover:bg-account-light text-white"
          text="Create Account query"
        />
      </div>

      <div className="w-full text-center my-8">
        <h1 className="text-account-dark font-bold text-3xl mb-1">
          Account ID
        </h1>
        <h1 className="text-sm w-[90%] mx-auto">
          Enter your Gamer tag first, and proceed to generate an Account which
          will consist of both public and private key
        </h1>
      </div>

      <div className="flex flex-col justify-between items-center">
        <InputEl disabled={true} labelText="Gamer Tag" placeholder="gamer tag:" />
        <InputEl disabled={true} labelText="Create Account ID" placeholder="Account ID:" />
      </div>
    </div>
  );
}
