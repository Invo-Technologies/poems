import { GiFeather, GiArrowCursor } from "react-icons/gi";
import { usePoemsContext } from "../../hooks/usePoemContext";
import { CgArrowLeftO } from 'react-icons/cg';

export const InputEl = ({
  showLabel,
  labelText,
  placeholder,
  disabled,
  className,
}) => {
  return (
    <div className="w-full mb-6">
      {showLabel && (
        <div
          className={`text-[12px] w-fit py-1 px-2 mb-4 w-fit rounded-md ${
            disabled ? "bg-[#3A414A] text-black" : className
          }`}
        >
          {labelText}
        </div>
      )}
      <input
        disabled={disabled}
        type="text"
        placeholder={placeholder}
        className="p-2 bg-transparent w-full border disabled:border-[#2C3137] outline-none rounded-md"
      />
      {/* <label className="label">
        <span className="label-text-alt">Bottom Right label</span>
      </label> */}
    </div>
  );
};

export const ButtonEl = ({ text, className, handleClick }) => {
  return (
    <button
      className={`border py-3 px-4 rounded-md transition-all duration-300 ${className}`}
      onClick={handleClick}
    >
      {text}
    </button>
  );
};

export const GenerationBtnCheck = ({ text, id }) => {
  return (
    <div onClick={() => alert("hello")}>
      <input type="checkbox" id={id} value="" className="hidden peer" />
      <label
        for={id}
        className="inline-flex items-center justify-between w-fit py-1 px-2 text-generation-dark bg-black
        rounded-md cursor-pointer hover:text-generation-dark peer-checked:text-generation-dark
        peer-checked:bg-white hover:bg-white"
      >
        <div className="block">
          <div className="w-full text">{text}</div>
        </div>
      </label>
    </div>
  );
};

export const DecryptBtnCheck = ({ text, id }) => {
  return (
    <div>
      <input type="checkbox" id={id} value="" className="hidden peer" />
      <label
        onClick={() => alert()}
        for={id}
        className="inline-flex items-center justify-between w-fit py-1 px-2 text-decrypt-dark bg-black
        rounded-md cursor-pointer hover:text-decrypt-dark peer-checked:text-decrypt-dark
        peer-checked:bg-white hover:bg-white"
      >
        <div className="block">
          <div className="w-full text">{text}</div>
        </div>
      </label>
    </div>
  );
};

export const AleoBtnCheck = ({ text, id }) => {
  return (
    <div>
      <input type="checkbox" id={id} value="" className="hidden peer" />
      <label
        for={id}
        className="inline-flex items-center justify-between w-fit py-1 px-2 text-aleo-dark bg-black
        rounded-md cursor-pointer hover:text-aleo-dark peer-checked:text-aleo-dark
        peer-checked:bg-white hover:bg-white"
      >
        <div className="block">
          <div className="w-full text">{text}</div>
        </div>
      </label>
    </div>
  );
};

export const PoemsLogo = ({ className }) => {
  return (
    <div className="w-fit flex items-end">
      <p className={`text-xl mr-1 ${className}`}>Poems</p>
      <GiFeather className={`text-4xl`} />
    </div>
  );
};

export const PrevButton = (props) => {
  const { steps } = usePoemsContext();
  const { activeStep } = usePoemsContext();
  const { stepss, setSteps, setActiveStep } = usePoemsContext();
  const goback = (step) => {
    for (let i = 0; i < step.length; i++) {
      if (step[i].isChecked === true) {
        if (activeStep !== "return") {
          let index = step.findIndex((x) => x.type === activeStep);
          if (index === 0) {
            setActiveStep(step[index].type);
            step.isChecked = false;
          } else {
            setActiveStep(step[index - 1].type);
            step.isChecked = false;
          }
        }
      }
    }
  };
  return (
    <button className="prevbutton" onClick={() => goback(props.steps)}>
      <CgArrowLeftO className={`text-2xl`} />
    </button>
  );
};
