import { usePoemsContext } from '../hooks/usePoemContext';
import { PoemsLogo } from '../shared/input';
import Progress from './Progress';
import Steps from './Steps';

export default function Header() {
  const { activeStep } = usePoemsContext;
  return (
    <div className="mb-8">
      <div className="flex justify-between items-center mb-6">
        <PoemsLogo />
        <Progress />
      </div>
      <div className="">
        <Steps />
      </div>
    </div>
  );
}
