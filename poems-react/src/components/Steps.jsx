import { usePoemsContext } from '../hooks/usePoemContext';
import { CgCheckO, CgRadioCheck } from 'react-icons/cg';

export default function Steps() {
  const { steps } = usePoemsContext();
  console.log(steps);
  return (
    <div className="flex justify-around my-4 items-center">
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
