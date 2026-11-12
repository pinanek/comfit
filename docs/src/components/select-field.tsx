"use client";

import { isPointerActivation } from "#/lib/pointer-activation";
import { Field } from "@base-ui/react/field";
import { Select } from "@base-ui/react/select";
import { Check, ChevronDown } from "lucide-react";
import { useState } from "react";
import styles from "./select-field.module.css";

export function SelectField<Value extends string>({ label, items, value, onValueChange, compact = false }: {
  label: string;
  compact?: boolean;
  items: { value: Value; label: string }[];
  value: Value;
  onValueChange: (value: Value) => void;
}) {
  const [pointerOpen, setPointerOpen] = useState(false);

  return (
    <Field.Root className={styles.field} data-compact={compact || undefined}>
      <Field.Label className={compact ? styles.hiddenLabel : undefined}>{label}</Field.Label>
      <Select.Root
        items={items}
        value={value}
        onOpenChange={(open, { event }) => setPointerOpen(open && isPointerActivation(event))}
        onValueChange={(next) => {
          if (next !== null) onValueChange(next);
        }}
      >
        <Select.Trigger className={`${styles.control} ${styles.selectTrigger}`}>
          <Select.Value className={styles.selectValue} />
          <Select.Icon className={styles.selectIcon}>
            <ChevronDown size={compact ? 12 : 16} aria-hidden="true" />
          </Select.Icon>
        </Select.Trigger>
        <Select.Portal>
          <Select.Positioner
            className={styles.selectPositioner}
            sideOffset={6}
            collisionPadding={12}
            alignItemWithTrigger
          >
            <Select.Popup className={styles.selectPopup} data-pointer-open={pointerOpen || undefined}>
              <Select.List>
                {items.map((item) => (
                  <Select.Item key={item.value} value={item.value} className={styles.selectItem}>
                    <Select.ItemText>{item.label}</Select.ItemText>
                    <Select.ItemIndicator className={styles.selectIcon}>
                      <Check size={16} aria-hidden="true" />
                    </Select.ItemIndicator>
                  </Select.Item>
                ))}
              </Select.List>
            </Select.Popup>
          </Select.Positioner>
        </Select.Portal>
      </Select.Root>
    </Field.Root>
  );
}
