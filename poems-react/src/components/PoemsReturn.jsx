import React, { useState, useEffect } from "react";
import { InputEl, ButtonEl, GenerationBtnCheck } from "../shared/input";
import { usePoemsContext } from "../hooks/usePoemContext";
import "../../css/styles.css";

const PoemsReturn = () => {
  const { steps, setSteps, setActiveStep } = usePoemsContext();
  const newSteps = steps.map((step) =>
    step.type === "return" ? { ...step, isChecked: true } : step
  );

  const handleNext = () => {
    setSteps(newSteps);
    setActiveStep("decrypt");
  };

  useEffect(() => {
    const timers = [
      setTimeout(() => {setShow(true), setLoad(false), setLoad1(true)}, 3750),
      setTimeout(() => {setShow1(true), setLoad1(false), setLoad2(true)}, 7500),
      setTimeout(() => {setShow2(true), setLoad2(false), setLoad3(true)}, 11250),
      setTimeout(() => {setShow3(true), setLoad3(true)}, 15000),
      setTimeout(() => {setShow4(true), setLoad3(false)}, 18750),
    ];

    return () => {
      timers.forEach((timer) => clearTimeout(timer));
    };
  }, []);

  const [load, setLoad] = useState(true)
  const [load1, setLoad1] = useState(false)
  const [load2, setLoad2] = useState(false)
  const [load3, setLoad3] = useState(false)
  const [load4, setLoad4] = useState(false)
  const [show, setShow] = useState(false);
  const [show1, setShow1] = useState(false);
  const [show2, setShow2] = useState(false);
  const [show3, setShow3] = useState(false);
  const [show4, setShow4] = useState(false);

  return (
    <>
      <div className='content p-7'>
        <div className='bars-container'>
          <div className='bar-text'>
            "BAR": Blind Asset Record
          </div>
          <div>
            <div
              className="bars w-full mt-2 bg-primary p-3"
              style={{ background: show ? "#9370db" : "" }}
            >
              Sending BAR
            </div>
          </div>
          <div>
            <div
              className="bars w-full mt-4 bg-primary p-3"
              style={{ background: show1 ? "#9370db" : "" }}
            >
              Committing BAR
            </div>
          </div>
          <div>
            <div
              className="bars w-full mt-4 bg-primary p-3"
              style={{ background: show2 ? "#9370db" : "" }}
            >
              Returning New BAR
            </div>
          </div>
          <div>
            <div
              className="bars w-full mt-4 bg-primary p-3"
              style={{ background: show3 ? "#9370db" : "" }}
            >
              Poem
            </div>
          </div>
        </div>

        <div className='loaders'>
          {load && (
            <div>
              <button href='' className='btn_form load1' type=''>
                <span></span>
                <span></span>
                <span></span>
                <span></span>
                loading <i className='bi-arrow-right'></i>
              </button>
            </div>
          )}

          {load1 && (
            <div>
              <button href='' className='btn_form load2' type=''>
                <span></span>
                <span></span>
                <span></span>
                <span></span>
                loading <i className='bi-arrow-right'></i>
              </button>
            </div>
          )}

          {load2 && (
            <div>
              <button href='' className='btn_form load3' type=''>
                <span></span>
                <span></span>
                <span></span>
                <span></span>
                loading <i className='bi-arrow-right'></i>
              </button>
            </div>
          )}

          {load3 && (
            <div>
              <button href='' className='btn_form load4' type=''>
                <span></span>
                <span></span>
                <span></span>
                <span></span>
                loading <i className='bi-arrow-right'></i>
              </button>
            </div>
          )}
        </div>
      </div>

      <div className="w-full poem mt-1 px-5">
        {show4 && (
          <>
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
            <div className="returnButton">
              <ButtonEl
                handleClick={handleNext}
                className="border border-white bg-aleo-dark hover:bg-aleo-light text-white"
                text="Save Interpretations"
              />
            </div>
          </>
        )}
      </div>
    </>
  );
};

export default PoemsReturn;
