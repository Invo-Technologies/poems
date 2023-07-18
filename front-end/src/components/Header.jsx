import Progress from './Progress';
import Steps from './Steps';

export default function Header() {
  return (
    <div className="">
      <div className="flex justify-between items-center">
        <div className="text-account-dark text-3xl font-bold">POEMS</div>
        <Progress />
      </div>
      <div className="">
        <Steps />
      </div>
    </div>
  );
}
