export const InputEl = ({ labelText, placeholder, disabled }) => {
  return (
    <div className="w-full mb-6">
      <div
        className={`text-[12px] w-fit py-1 px-2 mb-4 w-fit rounded-md ${
          disabled
            ? 'bg-[#3A414A] text-black'
            : 'bg-white text-black cursor-pointer'
        }`}
      >
        {labelText}
      </div>
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

export const ButtonEl = ({ text, className }) => {
  return (
    <button className={`border p-2 rounded-md ${className}`}>{text}</button>
  );
};
