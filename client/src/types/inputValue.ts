export interface IInputValue {
  value: string;
  invalid: boolean;
  error: string;
}

export function emptyInputValue(): IInputValue {
  return {
    value: '',
    invalid: false,
    error: '',
  };
}
