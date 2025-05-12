import "./index.css";
import { createRoot } from "react-dom/client";
import { useState, useEffect } from "react";
import { RackView, type Snapshot, type Step } from "logicline-view";
import { ModalBox } from "./modal/ModalBox";
import "../node_modules/logicline-view/dist/style.css";

const App = () => {
  const [data, setData] = useState<Snapshot | null>(null);
  const [isOpen, setIsOpen] = useState(false);
  const [step, setStep] = useState<Step | null>(null);

  useEffect(() => {
    let t: number;
    const fetchData = () => {
      fetch(`http://${window.location.hostname}:9001/state`)
        .then((res) => res.json())
        .then(setData)
        .catch((e: any) => {
          console.error(`Error fetching data: ${e}`);
          setData(null);
        });
      t = setTimeout(fetchData, 500);
    };
    fetchData();

    return () => clearTimeout(t);
  }, []);
  if (!data) {
    return <div>No data</div>;
  }
  return (
    <>
      <RackView
        data={data}
        onBlockClick={(v) => {
          setStep(v);
          setIsOpen(true);
        }}
      />
      <ModalBox isOpen={isOpen} step={step} onClose={() => setIsOpen(false)} />
    </>
  );
};

createRoot(document.getElementById("root")!).render(<App />);
