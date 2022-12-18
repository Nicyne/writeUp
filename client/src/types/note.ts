export type Allowance = 'Owner' | 'ReadWrite' | 'Read' | 'Forbidden';

export type Note = {
  note_id: string;
  note: {
    title: string;
    content: string;
    owner_id: string;
    tags: string[];
  };
  allowance: Allowance;
};

export type MetaNote = {
  note_id: string;
  title: string;
  tags: string[];
  allowance: Allowance;
};
