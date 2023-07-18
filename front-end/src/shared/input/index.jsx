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
            disabled ? 'bg-[#3A414A] text-black' : className
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
    <div>
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