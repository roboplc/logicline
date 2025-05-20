import React from "react";
import { useMemo } from "react";
import "./ModalBox.css";
import { type Step } from "logicline-view";
import { copyTextClipboard } from "bmat/dom";
import Icons from "../assets/icons/index";

interface ModalProps {
  isOpen: boolean;
  step: Step | null;
  onClose: () => void;
}

export const ModalBox: React.FC<ModalProps> = ({ isOpen, step, onClose }) => {
  if (!isOpen) return null;

  const formattedJson = useMemo(() => {
    return JSON.stringify(step?.input, null, 2);
  }, [step?.input]);

  const handleCopy = () => {
    copyTextClipboard(formattedJson);
  };

  return (
    <div className="logicline_modal-overlay" onClick={onClose}>
      <div className="logicline_modal-box" onClick={(e) => e.stopPropagation()}>
        <div className="logicline_modal-header">
          <h2 className="logicline_modal-title">Block input parameters</h2>
          <button className="logicline_modal-close" onClick={onClose}>
            {Icons.close}
          </button>
        </div>

        <div className="logicline_modal-content">
          <pre>{formattedJson}</pre>

          <button className="logicline_copy-button" onClick={handleCopy}>
            {Icons.copy}
          </button>
        </div>
      </div>
    </div>
  );
};
