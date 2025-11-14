export type NovoAgendamento = {
	cliente_id: number;
	data_hora: string | number;
	preco: number;
	concluido: boolean;
	servicos_ids: number[];
};

export type Agendamento = NovoAgendamento & { id: number };

export type Cliente = { id: number; nome: string };
export type Servico = { id: number; nome: string; preco: number; duracao_min?: number };
