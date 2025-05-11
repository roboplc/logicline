export enum InputKind {
  Flow = "flow",
  External = "external"
}

export interface Step {
  name: string;
  input: unknown;
  passed: boolean;
  input_kind?: InputKind;
}

export interface Line {
  name: string;
  steps: (Step | Step[])[];
}

export interface Snapshot {
  lines: {
    [key: string]: Line;
  };
}

export type BlockClickHandler = (step: Step) => void;
