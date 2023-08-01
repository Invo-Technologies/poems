import { useState } from 'react';
import { InputEl, ButtonEl } from '../shared/input';
import { usePoemsContext } from '../hooks/usePoemContext';

export default function AccountQuery() {
  const [showAcctID, setShowAcctID] = useState(false);
  const [currency, setCurrency] = useState('');
  const [asset, setAsset] = useState('');
  const [gamer, setGamer] = useState('');
  const [account, setAccount] = useState('');
  const [node, setNode] = useState('');
  const [game, setGame] = useState('');
  const [pool, setPool] = useState('');
  const { steps, setSteps, setActiveStep } = usePoemsContext();

  const newSteps = steps.map((step, _) => {
    if (step.type === 'account') {
      step.isChecked = true;
    }
    return step;
  });

  console.log(currency);

  const handleSubmit = e => {
    e.preventDefault();
    const data = {
      currency: currency,
      assetId: asset,
      gamerTag: gamer,
      accountId: account,
      nodeId: node,
      gameId: game,
      poolId: pool,
    };
    sessionStorage.setItem('userData', JSON.stringify(data));
  };
  const handleNext = () => {
    setSteps([...newSteps]);
    setActiveStep('generation');
  };
  return (
    <form onSubmit={handleSubmit}>
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
              // name='currency'
              value={currency}
              onChange={(e) => setCurrency(e.target.value)}
              placeholder="currency:"
            />
          </div>
          <div className="w-full md:w-[45%]">
            <InputEl
              className="bg-white text-black cursor-pointer"
              showLabel={true}
              disabled={true}
              labelText="Asset ID"
              // name='asset'
              value={asset}
              onChange={(e)=> setAsset(e.target.value)}
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
              // name='gamer'
              value={gamer}
              onChange={(e) => setGamer(e.target.value)}
              placeholder="gamer tag:"
            />
          </div>
          <div className="w-full md:w-[45%]">
            <InputEl
              showLabel={false}
              disabled={true}
              placeholder="Account ID:"
              // name='account'
              value={account}
              onChange={(e) => setAccount(e.target.value)}
            />
          </div>
        </div>
        <div className="flex flex-col justify-between items-center">
          <InputEl
            className="bg-white text-black cursor-pointer"
            showLabel={true}
            disabled={true}
            labelText="Node ID"
            // name='node'
            value={node}
            onChange={(e) => setNode(e.target.value)}
            placeholder="node id"
          />
          <InputEl
            className="bg-white text-black cursor-pointer"
            showLabel={true}
            disabled={true}
            labelText="Game ID"
            // name='game'
            value={game}
            onChange={(e) => setGame(e.target.value)}
            placeholder="game id"
          />
          <InputEl
            className="bg-white text-black cursor-pointer"
            showLabel={true}
            disabled={true}
            labelText="Pool ID"
            // name='pool'
            value={pool}
            onChange={(e) => setPool(e.target.value)}
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
                type="submit"
              />
            </div>
          </div>
        )}
      </div>
    </form>
  );
}
