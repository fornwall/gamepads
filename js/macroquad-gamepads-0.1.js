miniquad_add_plugin({name:"gamepads",version:.1,register_plugin:function(e,l){const u=8;const m=28;const p=.04;globalThis.addEventListener(`gamepadconnected`,()=>{});e.env.getGamepads=t=>{const e=navigator.getGamepads();const n=l?l.memory:wasm_memory;const a=new Float32Array(n.buffer);const s=new Uint32Array(n.buffer);const o=new Uint8Array(n.buffer);for(const[r,i]of e.slice(0,u).entries()){let e=t+m*r+1;if(!i||!i.connected||i.mapping!=="standard"){o[e]=0;continue}o[e++]=1;o[e++]=i.buttons.length;o[e++]=i.axes.length;let n=0;for(const[c,d]of i.buttons.entries()){if(c<17&&d.pressed)n|=1<<c}s[e/4]=n;e+=4;for(const[c,f]of i.axes.slice(0,4).entries()){const g=c===1||c===3?-1:1;a[e/4]=Math.abs(f)<p?0:g*(f-Math.sign(f)*p)/(1-p);e+=4}}};e.env.playEffect=(n,e,t,a,s)=>{const o=navigator.getGamepads().find(e=>e?.index===n);o?.vibrationActuator?.playEffect("dual-rumble",{duration:e,startDelay:t,strongMagnitude:a,weakMagnitude:s})}}});
