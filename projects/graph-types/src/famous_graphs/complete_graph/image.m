(* ::Package:: *)

SetDirectory@NotebookDirectory[];


kComplete[i_]:=HighlightGraph[#,GraphCenter[#]]&[CompleteGraph[i,VertexSize->Small,PlotLabel->Subscript[K,i]]];
dComplete[i_]:=HighlightGraph[#,GraphCenter[#]]&[CompleteGraph[i,DirectedEdges->True,VertexSize->Small,PlotLabel->Subscript[D,i]]];
Table[kComplete[i],{i,3,6}]
Table[dComplete[i],{i,3,6}]

(* as image row, save png *)
Export["k-complete.svg", GraphicsRow[Table[kComplete[i], {i, 3, 6}]], ImageSize -> 800]
Export["d-complete.svg", GraphicsRow[Table[dComplete[i], {i, 3, 6}]], ImageSize -> 800]



