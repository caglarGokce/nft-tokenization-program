import React from 'react';
import Stack from '../Stack';
import Icon from '../Icon';
import { IconDefinition, faWarning } from '@fortawesome/free-solid-svg-icons';
import Typography from '../Typography';
import Button from '../Button';

export type IconTextMessagePropsType = {
  /** Icon related to the message */
  icon?: IconDefinition;
  /** Title of the message */
  title: string;
  /** Further explanation of the message */
  description?: string;
  /** Properties for CTA button */
  action?: {
    /** Text of the button */
    text: string;
    /** Called on click of the button */
    onClick: () => void;
  };
};

/** Shows a message with an icon as well as a CTA. */
export default function IconTextMessage({
  icon,
  title,
  description,
  action,
}: Readonly<IconTextMessagePropsType>) {
  return (
    <Stack className="items-center justify-center" spacing={2}>
      <Icon size="xxl" icon={icon ?? faWarning} />
      <div className="mt-1"></div>
      <Typography variant="subtitle1" text={title} />
      {description && <Typography variant="body2" text={description} />}
      {action && (
        <Button
          variant="contained"
          onClick={action.onClick}
          text={action.text}
        />
      )}
    </Stack>
  );
}
