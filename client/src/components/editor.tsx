import { FunctionComponent, SyntheticEvent } from 'react';
import type { INote } from 'types';

interface props {
  note: INote;
}

export const Editor: FunctionComponent<props> = ({ note }) => {
  return (
    <div className="editor">
      <div className="editorInput">
        <header className="toolbar">
          <ul>
            <li className="widget">
              <button>Save</button>
            </li>
          </ul>
        </header>
        <textarea name="input" id="input" />
      </div>
      <div className="editorPreview"></div>
    </div>
  );
};
