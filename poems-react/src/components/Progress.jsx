import { usePoemsContext } from '../hooks/usePoemContext';

export default function Progress() {
  const { steps, activeStep } = usePoemsContext();
  const total = steps.length;
  const completed = steps.reduce((aggr, curr, index) => {
    if (steps[index].isChecked) {
      aggr += 1;
    }
    return aggr;
  }, 0);

  return (
    <div className="w-28 md:w-64">
      <div className="text-xs mb-1">
        {completed} of {total} complete
      </div>
      <div className="h-5 w-full bg-neutral-200 dark:bg-neutral-600 rounded-full overflow-hidden">
        <div
          className={`h-full ${
            activeStep === 'account'
              ? `bg-account-dark`
              : activeStep === 'generation'
              ? `bg-generation-dark`
              : activeStep === 'aleo'
              ? `bg-aleo-dark`
              : activeStep === 'return'
              ? `bg-return-dark`
              : `bg-decrypt-dark`
          }
                `}
          style={{ width: `${(completed / total) * 100}%` }}
        ></div>
      </div>
    </div>
  );
}
