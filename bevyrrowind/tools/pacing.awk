# Frame-pacing classifier for perf CSVs (perf.md "Frame pacing"). usage: gawk -F, [-v verbose=1] -f tools/pacing.awk file.csv
NR==1 { for (i=1;i<=NF;i++) c[$i]=i; next }
{ n++; f[n]=$c["frame_ms"]; 
  pr[n]= ("render/present/ms" in c) ? $c["render/present/ms"] : $c["render/submit_present/ms"];
  aq[n]=$c["render/acquire/ms"]+0; th[n]=$c["render/thread/ms"]+0;
  wk[n]= ("frame/work/ms" in c) ? $c["frame/work/ms"] : "";
  mb[n]= ("main/busy/ms" in c) ? $c["main/busy/ms"] : "";
  rb[n]= ("render/busy/ms" in c) ? $c["render/busy/ms"] : "";
  ms[n]=$c["assets/Mesh/added"]+0; sp[n]=$c["stream/spawn/items"]+$c["stream/despawn/items"];
  os[n]=$c["world/origin_shifts"]+0 }
function med(a,  i,m,t){ m=0; for(i=1;i<=n;i++) if (a[i]!="") t[++m]=a[i]+0; asort(t); return m? t[int((m+1)/2)] : 0 }
# stutters: frames > 1.5 x median of the previous 60 finite values (sf_diag pacing::stutters)
function stut(a, r, fl,  i,j,k,m,t,cnt,q,h){ cnt=0; h=0
  for(i=1;i<=n;i++){ if (a[i]=="" || r[i]=="") continue
    if (h>=8){ m=0; delete t; for(j=1;j<=h;j++) t[++m]=q[j]; asort(t); k=(m%2)? t[(m+1)/2] : (t[m/2]+t[m/2+1])/2; if (a[i]+0>1.5*k) { cnt++; fl[i]=1 } }
    if (h==60){ for(j=1;j<60;j++) q[j]=q[j+1]; h=59 }
    q[++h]=r[i]+0 }
  return cnt }
function pct(a,p,  i,m,t){ m=0; for(i=1;i<=n;i++) if (a[i]!="") t[++m]=a[i]+0; asort(t); return m? t[int(p*(m-1))+1] : 0 }
END {
  M=med(f); P=med(pr); W=med(wk); thr=1.5*M; worst=0; wworst=0
  for(i=1;i<=n;i++){ if(f[i]>worst)worst=f[i]; if(wk[i]!="" && wk[i]+0>wworst)wworst=wk[i]+0 }
  for(i=1;i<=n;i++){ if (wk[i]!="" && W>0 && wk[i]+0>1.5*W) nw++
    if (f[i]<=thr) continue; ns++
    shift = os[i]+os[i-1]+os[i+1]+os[i+2] > 0
    churn = ms[i-1]+ms[i] > 50 || sp[i-1]+sp[i] > 300
    present = pr[i]+0 > 2*P+2
    if (shift) k="shift"; else if (churn) k="stream"; else if (present) k="present"; else k="other"
    cls[k]++
    if (verbose) printf "  f%d %.1f pres %.1f acq %.1f thr %.1f work %s main %s rbusy %s mesh %d sp %d sh %d -> %s\n", i, f[i], pr[i], aq[i], th[i], wk[i], mb[i], rb[i], ms[i-1]+ms[i], sp[i-1]+sp[i], os[i-1]+os[i]+os[i+1]+os[i+2], k
  }
  printf "%s median %.2f worst %.1f (%.2fx) over1.5x %d [shift %d stream %d present %d other %d]", FILENAME, M, worst, worst/M, ns, cls["shift"], cls["stream"], cls["present"], cls["other"]
  if (W>0) printf " | work median %.2f worst %.1f (%.2fx) over %d", W, wworst, wworst/W, nw
  dur=0; for(i=1;i<=n;i++) dur+=f[i]; mins=dur/60000
  sf=stut(f,f,FF); sw=(W>0? stut(wk,wk,WF) : -1); swf=(W>0? stut(wk,f,XF) : -1); both=0; for(i=1;i<=n;i++) if (FF[i] && (WF[i-1] || WF[i] || WF[i+1])) both++
  printf " | stut/min frame %.1f work %.1f work>frame %.1f frame+work %.1f | p99.9 %.1f", sf/mins, sw/mins, swf/mins, both/mins, pct(f,0.999)
  printf "\n"
}
