#!/bin/bash

for example in $(find examples-cn -type d -name "*"); do
  html=stage/_book/${example#examples-cn/}.html
  if [[ -f ${html} ]]; then
    echo ${html}

    sed -i -e s:${example#examples-cn/}.md:${example}/input.md: ${html}
  fi
done
