import { FunctionComponent } from 'react';
import { EditorContextProvider } from 'context';
import {
  Sidebar,
  BodyEditor,
  MenuStrip,
  TitleEditor,
  TagEditor,
  Preview,
} from './';

/**
 * Features missing from legacy editor:
 * - menuStrip
 *  - [x] save note
 *  - [x] close note
 *  - [x] refresh notes
 * - sideBar
 *  - [x] load notes
 *  - [x] open note
 *  - [] create note
 *  - [x] delete note
 * - editor
 *  - [x] body change
 *  - [x] brackets auto close
 *  - [] auto save
 *  - [x] markdown preview
 * - widgetBar
 *  - [x] insert snippets
 *
 * Features missing generally
 * - editor
 *  - [x] edit title
 *  - [x] edit tags
 * - sidebar
 *  - [] search
 *  - [x] tag sorting
 * - general
 *  - [] change allowances
 *
 * - [] Styling
 */

/* TOP LEVEL COMPONENT FOR EDITOR */
export const Editor: FunctionComponent = () => {
  return (
    <EditorContextProvider>
      <MenuStrip />
      <Sidebar />
      <div>
        <TitleEditor />
        <BodyEditor />
        <TagEditor />
        <Preview />
      </div>
    </EditorContextProvider>
  );
};
